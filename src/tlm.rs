use crate::Packet;

pub trait Telemetry {
    fn to_packet(&self) -> Packet;
}