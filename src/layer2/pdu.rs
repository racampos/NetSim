use super::address::MacAddress;
use super::arp::{ArpOperation, ArpPacket};
use crate::layer1::crc::crc32;
use crate::layer3::{
    address::Ipv4Addr,
    pdu::{Ipv4Packet, Ipv6Packet},
};
use bevy::prelude::*;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Component, Debug)]
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

    pub fn new_arp(src: MacAddress, sender_ip: Ipv4Addr, target_ip: Ipv4Addr) -> Self {
        let mut frame = Self::new(src.clone(), MacAddress::broadcast());
        frame.payload = EthernetPayload::ARP(ArpPacket::new(
            ArpOperation::Request,
            src,
            sender_ip,
            target_ip,
        ));
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
