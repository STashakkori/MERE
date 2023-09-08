use crate::pkt::PacketTrait;

pub struct Executive;

impl Executive {
    pub fn new() -> Self {
        return Executive;
    }

    pub fn execute_commands(&self, packet_buffer: Vec<Box<dyn PacketTrait>>) {
        for packet in packet_buffer {
            packet.execute();
        }
    }

    pub fn terminate(&self) {
        println!("Terminating...");
        std::process::exit(0); // Exit program
    }
}