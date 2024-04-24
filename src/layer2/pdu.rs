use super::address::MacAddress;
use super::arp::{ArpOperation, ArpPacket};
use crate::layer1::crc::crc32;
use crate::layer3::{
    address::Ipv4Addr,
    pdu::{Ipv4Packet, Ipv6Packet},
};
use bevy::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum EthernetPayload {
    IPv4(Ipv4Packet),
    IPv6(Ipv6Packet),
    ICMP,
    ARP(ArpPacket),
    Dummy,
}

impl EthernetPayload {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            EthernetPayload::IPv4(packet) => packet.to_bytes(),
            EthernetPayload::IPv6(packet) => unimplemented!(),
            EthernetPayload::ICMP => unimplemented!(),
            EthernetPayload::ARP(arp_packet) => arp_packet.to_bytes(),
            EthernetPayload::Dummy => Vec::new(),
        }
    }
}

impl fmt::Display for EthernetPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EthernetPayload::IPv4(packet) => write!(f, "{}", packet),
            EthernetPayload::IPv6(packet) => write!(f, "{}", packet),
            EthernetPayload::ICMP => write!(f, "ICMP"),
            EthernetPayload::ARP(packet) => write!(f, "{}", packet),
            EthernetPayload::Dummy => write!(f, "Dummy Payload"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VlanTag {
    tpid: [u8; 2],
    pcp: u8,
    dei: u8,
    vid: [u8; 2],
}

impl VlanTag {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.tpid);
        let pcp_dei = (self.pcp << 5) | self.dei;
        bytes.push(pcp_dei);
        bytes.extend_from_slice(&self.vid);
        bytes
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Ethertype {
    IPv4,    // 0x0800
    IPv6,    // 0x86DD
    ARP,     // 0x0806
    Unknown, // 0x0000
}

impl Ethertype {
    pub fn get_value(&self) -> [u8; 2] {
        match self {
            Ethertype::IPv4 => [0x08, 0x00],
            Ethertype::IPv6 => [0x86, 0xDD],
            Ethertype::ARP => [0x08, 0x06],
            Ethertype::Unknown => [0x00, 0x00],
        }
    }
}

impl fmt::Display for Ethertype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ARP")
    }
}

#[derive(Component, Debug, Clone)]
pub struct EthernetFrame {
    pub dest: MacAddress,
    pub src: MacAddress,
    pub vlan: Option<VlanTag>,
    pub ethertype: Ethertype,
    pub payload: EthernetPayload,
    pub fcs: [u8; 4],
}

impl EthernetFrame {
    pub fn new(src: MacAddress, dest: MacAddress) -> Self {
        Self {
            dest,
            src,
            vlan: None,
            ethertype: Ethertype::Unknown,
            payload: EthernetPayload::Dummy,
            fcs: [0; 4],
        }
    }

    pub fn arp_request(src: MacAddress, sender_ip: Ipv4Addr, target_ip: Ipv4Addr) -> Self {
        let mut frame = Self::new(src.clone(), MacAddress::broadcast());
        frame.ethertype = Ethertype::ARP;
        frame.payload = EthernetPayload::ARP(ArpPacket::new(
            ArpOperation::Request,
            src,
            sender_ip,
            target_ip,
        ));
        frame.fcs = crc32(&frame.to_bytes());
        frame
    }

    pub fn arp_reply(&self, arp: &ArpPacket, sender_mac: MacAddress) -> Self {
        let arp_reply = arp.create_reply(sender_mac);
        let payload = EthernetPayload::ARP(arp_reply.clone());
        let mut frame = Self::new(arp_reply.sender_mac, arp_reply.target_mac);
        frame.payload = payload;
        frame.ethertype = Ethertype::ARP;
        frame.fcs = crc32(&frame.to_bytes());
        frame
    }

    // Converts the Ethernet frame to a byte vector excluding the FCS
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.dest.to_bytes());
        bytes.extend_from_slice(&self.src.to_bytes());
        if let Some(vlan) = &self.vlan {
            bytes.extend_from_slice(&vlan.to_bytes());
        }
        bytes.extend_from_slice(&self.ethertype.get_value());
        bytes.extend_from_slice(&self.payload.to_bytes());
        bytes
    }
}

impl fmt::Display for EthernetFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EthernetFrame\n\
            \tdest: MacAddress: {}\n\
            \tsrc: MacAddress: {}\n\
            \tvlan: {:?}\n\
            \tethertype: {}\n\
            \tpayload: {}\n\
            \tfcs: [{}, {}, {}, {}]",
            self.dest,
            self.src,
            self.vlan,
            self.ethertype,
            self.payload,
            self.fcs[0],
            self.fcs[1],
            self.fcs[2],
            self.fcs[3]
        )
    }
}
