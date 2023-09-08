use crate::chron::Timestamp;

pub struct Packet {
    version: u8,
    packet_type: PacketType,
    source_id: u16,
    destination_id: u16,
    timestamp: Timestamp,
    payload: Vec<u8>
}

pub trait PacketTrait {
    fn execute(&self);
    fn serialize(&self) -> Vec<u8>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PacketType {
    Command,
    Telemetry,
    Housekeeping
}

impl Packet {
    const DEFAULT_VERSION: u8 = 2; // Version 1 reserved for dev

    // Default Constructor
    pub fn default() -> Self {
        let timestamp = Timestamp::take();
        Packet {
            version: Self::DEFAULT_VERSION,
            packet_type: PacketType::Housekeeping,
            source_id: 0,
            destination_id: 0,
            timestamp,
            payload: Vec::new()
        }
    }
                                                                               
    // Constructor for packets
    pub fn new(version: u8,
               packet_type: PacketType,
               source_id: u16, 
               destination_id: u16,
               payload: Vec<u8>) -> Self
    {
        let timestamp = Timestamp::take();
        Packet { version, packet_type, source_id, timestamp, destination_id, payload }
    }

    // Constructor for deserialization
    pub fn from_u8(bytes: Vec<u8>) -> Result<Self, &'static str> {
        if bytes.len() < 50 { return Err("Invalid packet length"); }
        let timestamp = Timestamp::take();
        let version = bytes[0];
        let packet_type_value = bytes[1] & 0x0F;
        let packet_type = match packet_type_value {
            0 => PacketType::Command,
            1 => PacketType::Telemetry,
            _ => return Err("Invalid packet type value"),
        };
        let source_id = u16::from_be_bytes([bytes[2], bytes[3]]);
        let destination_id = u16::from_be_bytes([bytes[4], bytes[5]]);
        let timestamp_seconds =  i64::from_be_bytes([bytes[6],  bytes[7],
                                                     bytes[8],  bytes[9],
                                                     bytes[10], bytes[11],
                                                     bytes[12], bytes[13]]);
        let timestamp_fraction = i64::from_be_bytes([bytes[14], bytes[15],
                                                     bytes[16], bytes[17],
                                                     bytes[18], bytes[19],
                                                     bytes[20], bytes[21]]);

        let payload = bytes[14..].to_vec();

        return Ok(Packet::new(version, packet_type, source_id, destination_id, payload))
    }

    // Getters
    pub fn get_version(&self) -> u8 { return self.version }
    pub fn get_packet_type(&self) -> PacketType { return self.packet_type }
    pub fn get_source_id(&self) -> u16 { return self.source_id }
    pub fn get_destination_id(&self) -> u16 { return self.destination_id }
    pub fn get_timestamp_seconds(&self) -> i64 { return self.timestamp.get_seconds() }
    pub fn get_timestamp_fractional(&self) -> i64 { return self.timestamp.get_fractional() }
    pub fn get_payload(&self) -> &[u8] { return &self.payload }
}

impl PacketTrait for Packet {
    fn execute(&self) { }

    fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        serialized.push(self.version);
        match self.packet_type {
            PacketType::Command => serialized.push(0 as u8),
            PacketType::Telemetry => serialized.push(1 as u8),
            PacketType::Housekeeping => serialized.push(2 as u8)
        }
        serialized.extend_from_slice(&self.source_id.to_be_bytes());
        serialized.extend_from_slice(&self.destination_id.to_be_bytes());
        serialized.extend_from_slice(&self.timestamp.get_seconds().to_be_bytes());
        serialized.extend_from_slice(&self.timestamp.get_fractional().to_be_bytes());
        serialized.extend_from_slice(&self.payload);
        return serialized
    }
}