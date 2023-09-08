// Modular Embedded Rust Executive (MERE)
//
// By: Sina Tashakkori, QVLx Labs
//
mod exec;
use exec::Executive;
mod pkt;
mod pktin; use pktin::PacketIngest;
mod cmds; pub use cmds::StatusCommand;
mod chron;

fn main() {
    println!("Running MER");
    let executive = Executive::new();

    loop {
        let packet_buffer = PacketIngest::get_packets_test();

        executive.execute_commands(packet_buffer);
        executive.terminate();
    };
}

#[test]
fn test_cmdpkt_create() {
    use pkt::PacketType;
    let test_statcmd = StatusCommand::new(1, PacketType::Command, 456, 789, vec![3,2,1]);
    assert_eq!(test_statcmd.packet.get_version(), 1);
    assert_eq!(test_statcmd.packet.get_packet_type(), PacketType::Command);
    assert_eq!(test_statcmd.packet.get_source_id(), 456);
    assert_eq!(test_statcmd.packet.get_destination_id(), 789);
    assert_eq!(test_statcmd.packet.get_payload(), &[3, 2, 1]);
}

#[test]
fn test_cmdpkt_serialize() {
    use pkt::{PacketType, PacketTrait};
                           // version, pktType, srcID, dstID, timestamp(s,subs), payload
    let test_pkt = StatusCommand::new(1, PacketType::Command, 456, 789, vec![4, 3, 2, 1]);
    let serialized_pkt = test_pkt.serialize();
    let expected: Vec<u8> = vec![
        1,           // version as u8
        0,           // packet_type of Command as u8
        1, 200,      // source_id of 456 as u16
        3, 21,       // destination_id of 789 as u16
        0, 0, 0, 0,  // timestamp seconds as i64
        0, 0, 0, 0,  // timestamp fractional as i64
        4, 3, 2, 1   // 22B header ... payload as vec[u8]
    ];

    assert_eq!(serialized_pkt[..6], expected[..6]); // Compare only the first 8 bytes
    assert_eq!(serialized_pkt[14..], expected[14..]); // Compare the payload part
}

#[test]
fn test_cmdpkt_deserialize() {
    use pkt::{Packet, PacketType, PacketTrait};
    let expected_pkt = StatusCommand::new(1, PacketType::Command, 456, 789, vec![3, 2, 1]);
    let serialized_pkt = expected_pkt.serialize();
    
    if let Ok(deserialized_pkt) = StatusCommand::from_u8(serialized_pkt.clone()) {
        assert_eq!(deserialized_pkt.packet.get_version(), expected_pkt.packet.get_version(), "Version mismatch");
        assert_eq!(deserialized_pkt.packet.get_packet_type(), expected_pkt.packet.get_packet_type(), "Packet type mismatch");
        assert_eq!(deserialized_pkt.packet.get_source_id(), expected_pkt.packet.get_source_id(), "Source ID mismatch");
        assert_eq!(deserialized_pkt.packet.get_destination_id(), expected_pkt.packet.get_destination_id(), "Destination ID mismatch");
        assert_ne!(deserialized_pkt.packet.get_timestamp_seconds(), 0, "Timestamp is zero");
        assert_ne!(deserialized_pkt.packet.get_timestamp_fractional(), 0, "Timestamp fractional is zero");
        assert_eq!(deserialized_pkt.packet.get_payload(), expected_pkt.packet.get_payload(), "Payload mismatch");
    } else {
        assert!(false, "Deserialization failed");
    }
}