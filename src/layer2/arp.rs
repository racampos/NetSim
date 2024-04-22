use super::{address::MacAddress, pdu::Ethertype};
use crate::layer3::address::Ipv4Addr;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
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

impl fmt::Display for ArpOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArpOperation::Request => write!(f, "Request (0x0001)"),
            ArpOperation::Reply => write!(f, "Reply (0x0002)"),
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

impl fmt::Display for ArpHardwareType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ArpHardwareType::Ethernet => write!(f, "Ethernet (0x0001)"),
        }
    }
}

#[derive(Debug, Clone)]
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

impl fmt::Display for ArpPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ARP Packet:\n\
            \t\tHardware Type: {}\n\
            \t\tProtocol Type: {}\n\
            \t\tHardware Size: {}\n\
            \t\tProtocol Size: {}\n\
            \t\tOperation: {}\n\
            \t\tSender MAC: {}\n\
            \t\tSender IP: {}\n\
            \t\tTarget MAC: {}\n\
            \t\tTarget IP: {}",
            self.hardware_type,
            self.protocol_type,
            self.hardware_size,
            self.protocol_size,
            self.operation,
            self.sender_mac,
            self.sender_ip,
            self.target_mac,
            self.target_ip
        )
    }
}

#[derive(Debug)]
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

impl fmt::Display for ArpTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Determine the maximum width of the IP addresses for alignment
        let max_ip_width = self
            .entries
            .keys()
            .map(|ip| ip.to_string().len())
            .max()
            .unwrap_or(0);

        // Create a header
        let header = format!("{:<max_ip_width$} | MAC Address", "IP Address");
        writeln!(f, "{}", header)?;
        writeln!(f, "{}", "-".repeat(header.len()))?;

        // Print each entry
        for (ip, mac) in &self.entries {
            writeln!(f, "{:<max_ip_width$} | {}", ip, mac)?;
        }

        Ok(())
    }
}
