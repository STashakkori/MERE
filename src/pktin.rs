use crate::pkt::PacketTrait;
use crate::cmds::{ StatusCommand, ExitCommand };

use crate::pkt::PacketType;

pub struct PacketIngest;

impl PacketIngest {
    pub fn get_packets_test() -> Vec<Box<dyn PacketTrait>> {
        let mut packet_buffer: Vec<Box<dyn PacketTrait>> = Vec::new();

        let status_command = StatusCommand::new(1, PacketType::Command, 456, 23, vec![1,2,3]);
        let exit_command = ExitCommand::new(1, PacketType::Command, 456, 23, vec![1,2,3]);

        packet_buffer.push(Box::new(status_command));
        packet_buffer.push(Box::new(exit_command));

        return packet_buffer;
    }

    // This needs to be in Injest
    /*
    pub fn process(&self) {
        match self.packet_type {
            PacketType::Telecommand => {
                // Process telecommand payload
                // ...
            }
            PacketType::Telemetry => {
                // Process telemetry payload
                // ...
            }
        }
    }
    */
}