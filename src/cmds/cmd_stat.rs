use crate::pkt::{Packet,PacketTrait,PacketType};

pub struct StatusCommand {
    pub packet: Packet   
}

impl StatusCommand {
    pub fn new(version: u8,
               packet_type: PacketType,
               source_id: u16, 
               destination_id: u16,
               payload: Vec<u8>) -> Self {
        let packet = Packet::new(version, packet_type, source_id, destination_id, payload);
        StatusCommand { packet }
    }

    pub fn from_u8(bytes: Vec<u8>) -> Result<Self, &'static str> {
        let packet = Packet::from_u8(bytes)?;
        Ok(StatusCommand { packet })
    }
}

impl PacketTrait for StatusCommand {
    fn execute(&self) {
        println!("Executing StatusCommand...");
    }

    fn serialize(&self) -> Vec<u8> {
        // Implement the serialization logic for StatusCommand
        println!("Serializing StatusCommand...");
        return self.packet.serialize();
    }
}