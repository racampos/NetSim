use super::{address::MacAddress, pdu::Ethertype};
use crate::layer3::address::Ipv4Addr;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ArpOperation {
    Request, // 0x0001
    Reply,   // 0x0002
}
impl ArpOperation {
    pub fn get_value(&self) -> [u8; 2] {
        match self {
            ArpOperation::Request => [0x00, 0x01],
            ArpOperation::Reply => [0x00, 0x02],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArpHardwareType {
    Ethernet, // 0x0001
}

impl ArpHardwareType {
    pub fn get_value(&self) -> [u8; 2] {
        match self {
            ArpHardwareType::Ethernet => [0x00, 0x01],
        }
    }
}

#[derive(Debug)]
pub struct ArpPacket {
    pub hardware_type: ArpHardwareType,
    pub protocol_type: Ethertype,
    pub hardware_size: u8,
    pub protocol_size: u8,
    pub operation: ArpOperation,
    pub sender_mac: MacAddress,
    pub sender_ip: Ipv4Addr,
    pub target_mac: MacAddress,
    pub target_ip: Ipv4Addr,
}

impl ArpPacket {
    pub fn new(
        operation: ArpOperation,
        sender_mac: MacAddress,
        sender_ip: Ipv4Addr,
        target_ip: Ipv4Addr,
    ) -> Self {
        Self {
            hardware_type: ArpHardwareType::Ethernet,
            protocol_type: Ethertype::IPv4,
            hardware_size: 6, // MAC address length in octets
            protocol_size: 4, // IPv4 length in octets
            operation,
            sender_mac,
            sender_ip,
            target_mac: MacAddress::new("00:00:00:00:00:00".to_string()).unwrap(), // We don't know the target MAC address yet
            target_ip,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.hardware_type.get_value());
        bytes.extend_from_slice(&self.protocol_type.get_value());
        bytes.push(self.hardware_size);
        bytes.push(self.protocol_size);
        bytes.extend_from_slice(&self.operation.get_value());
        bytes.extend_from_slice(&self.sender_mac.to_bytes());
        bytes.extend_from_slice(&self.sender_ip.to_bytes());
        bytes.extend_from_slice(&self.target_mac.to_bytes());
        bytes.extend_from_slice(&self.target_ip.to_bytes());
        bytes
    }

    pub fn create_reply(&self, sender_mac: MacAddress) -> Self {
        Self {
            hardware_type: self.hardware_type,
            protocol_type: self.protocol_type,
            hardware_size: self.hardware_size,
            protocol_size: self.protocol_size,
            operation: ArpOperation::Reply,
            sender_mac: sender_mac,
            sender_ip: self.target_ip.clone(),
            target_mac: self.sender_mac.clone(),
            target_ip: self.sender_ip.clone(),
        }
    }
}

pub struct ArpTable {
    entries: HashMap<Ipv4Addr, MacAddress>,
}

impl ArpTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, ip: Ipv4Addr, mac: MacAddress) {
        self.entries.insert(ip, mac);
    }

    pub fn get_mac_address(&self, ip: &Ipv4Addr) -> Option<MacAddress> {
        self.entries.get(ip).cloned()
    }
}
