use super::address::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::layer2::address::MacAddress;
use bevy::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum Protocols {
    ICMP,
    IGMP,
    TCP,
    UDP,
    GRE,
    ESP,
    AH,
    EIGRP,
    OSPF,
    PIM,
    VRRP,
    L2TP,
    ISIS,
    MPLS,
    Unknown,
}

impl Protocols {
    pub fn get_value(&self) -> u8 {
        match self {
            Protocols::ICMP => 1,
            Protocols::IGMP => 2,
            Protocols::TCP => 6,
            Protocols::UDP => 17,
            Protocols::GRE => 47,
            Protocols::ESP => 50,
            Protocols::AH => 51,
            Protocols::EIGRP => 88,
            Protocols::OSPF => 89,
            Protocols::PIM => 103,
            Protocols::VRRP => 112,
            Protocols::L2TP => 115,
            Protocols::ISIS => 124,
            Protocols::MPLS => 137,
            Protocols::Unknown => 0,
        }
    }
}

impl fmt::Display for Protocols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocols::ICMP => write!(f, "ICMP"),
            Protocols::IGMP => write!(f, "IGMP"),
            Protocols::TCP => write!(f, "TCP"),
            Protocols::UDP => write!(f, "UDP"),
            Protocols::GRE => write!(f, "GRE"),
            Protocols::ESP => write!(f, "ESP"),
            Protocols::AH => write!(f, "AH"),
            Protocols::EIGRP => write!(f, "EIGRP"),
            Protocols::OSPF => write!(f, "OSPF"),
            Protocols::PIM => write!(f, "PIM"),
            Protocols::VRRP => write!(f, "VRRP"),
            Protocols::L2TP => write!(f, "L2TP"),
            Protocols::ISIS => write!(f, "ISIS"),
            Protocols::MPLS => write!(f, "MPLS"),
            Protocols::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct IpPayload {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct Ipv4Header {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: Protocols,
    pub header_checksum: u16,
    pub src: Ipv4Addr,
    pub dest: Ipv4Addr,
}

impl fmt::Display for Ipv4Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IPv4Header {{ version: {}, IHL: {}, DSCP: {}, ECN: {}, length: {}, id: {}, flags: {}, offset: {}, TTL: {}, protocol: {}, checksum: {:04X}, src: {}, dest: {} }}",
            self.version,
            self.ihl,
            self.dscp,
            self.ecn,
            self.total_length,
            self.identification,
            self.flags,
            self.fragment_offset,
            self.ttl,
            self.protocol,
            self.header_checksum,
            self.src,
            self.dest
        )
    }
}

#[derive(Debug)]
pub struct Ipv4Packet {
    pub header: Ipv4Header,
    pub payload: IpPayload,
}

impl Ipv4Packet {
    pub fn new(src: Ipv4Addr, dest: Ipv4Addr, payload: IpPayload) -> Self {
        Self {
            header: Ipv4Header {
                version: 4,
                ihl: 5,
                dscp: 0,
                ecn: 0,
                total_length: 0,
                identification: 0,
                flags: 0,
                fragment_offset: 0,
                ttl: 255,
                protocol: Protocols::TCP,
                header_checksum: 0,
                src,
                dest,
            },
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push((self.header.version << 4) | self.header.ihl);
        bytes.push((self.header.dscp << 2) | self.header.ecn);
        bytes.push((self.header.total_length >> 8) as u8);
        bytes.push((self.header.identification >> 8) as u8);
        bytes.push(self.header.identification as u8);
        bytes.push((self.header.flags << 5) | (self.header.fragment_offset >> 8) as u8);
        bytes.push((self.header.fragment_offset >> 8) as u8);
        bytes.push(self.header.fragment_offset as u8);
        bytes.push((self.header.ttl) as u8);
        bytes.push(self.header.protocol.get_value());
        bytes.push((self.header.header_checksum >> 8) as u8);
        bytes.push(self.header.header_checksum as u8);
        bytes.extend_from_slice(&self.header.src.to_bytes());
        bytes.extend_from_slice(&self.header.dest.to_bytes());
        bytes.extend_from_slice(&self.payload.data);
        bytes
    }
}

impl fmt::Display for Ipv4Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IPv4Packet {{ header: {}}}", self.header)
    }
}

#[derive(Debug)]
pub struct Ipv6Packet {
    pub src: Ipv6Addr,
    pub dest: Ipv6Addr,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub protocol: Protocols,
    pub payload_length: Option<u16>,
    pub flow_label: Option<u32>,
    pub next_header: Option<u8>, // Assuming a similar optional field as Python's None.
    pub payload: IpPayload,
}

impl Ipv6Packet {
    pub fn new(
        src: Ipv6Addr,
        dest: Ipv6Addr,
        payload: IpPayload,
        traffic_class: u8,
        hop_limit: u8,
        protocol: Protocols,
    ) -> Self {
        Self {
            src,
            dest,
            traffic_class,
            hop_limit,
            protocol,
            payload_length: None,
            flow_label: None,
            next_header: None,
            payload,
        }
    }
}

impl fmt::Display for Ipv6Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IPv6Packet {{ src: {}, dest: {}, traffic_class: {:02X}, hop_limit: {}, protocol: {}, payload_length: {:?}, flow_label: {:?}, next_header: {:?} }}",
            self.src,
            self.dest,
            self.traffic_class,
            self.hop_limit,
            self.protocol,
            self.payload_length,
            self.flow_label,
            self.next_header
        )
    }
}

#[derive(Debug)]
pub enum IpPacket {
    V4(Ipv4Packet),
    V6(Ipv6Packet),
}

impl IpPacket {
    pub fn new(src: IpAddr, dest: IpAddr, payload: IpPayload, protocol: Protocols) -> Self {
        match src {
            IpAddr::V4(src) => match dest {
                IpAddr::V4(dest) => IpPacket::V4(Ipv4Packet::new(src, dest, payload)),
                IpAddr::V6(_) => {
                    panic!("Destination address is not an IPv4 address");
                }
            },
            IpAddr::V6(src) => match dest {
                IpAddr::V6(dest) => {
                    IpPacket::V6(Ipv6Packet::new(src, dest, payload, 0, 0, protocol))
                }
                IpAddr::V4(_) => {
                    panic!("Destination address is not an IPv6 address");
                }
            },
        }
    }
}
