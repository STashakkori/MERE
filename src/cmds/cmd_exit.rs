use crate::pkt::{Packet, PacketTrait, PacketType};

pub struct ExitCommand {
    pub packet: Packet   
}

impl ExitCommand {
    pub fn new(version: u8,
               packet_type: PacketType,
               source_id: u16, 
               destination_id: u16,
               payload: Vec<u8>) -> Self {
        let packet = Packet::new(version, packet_type, source_id, destination_id, payload);
        ExitCommand { packet }
    }

    pub fn from_u8(bytes: Vec<u8>) -> Result<Self, &'static str> {
        let packet = Packet::from_u8(bytes)?;
        Ok(ExitCommand { packet })
    }
}

impl PacketTrait for ExitCommand {
    fn execute(&self) {
        // Implement the execute logic for ExitCommand
        println!("Executing ExitCommand...");
    }

    fn serialize(&self) -> Vec<u8> {
        // Implement the serialization logic for ExitCommand
        println!("Serializing ExitCommand...");
        return self.packet.serialize();
    }
}