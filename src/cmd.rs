use crate::Packet;

pub trait Command {
    fn execute(&self);
}