use bevy::prelude::*;
use super::address::MacAddress;
use crate::layer3::pdu::IpPacket;

#[derive(Debug)]
pub enum EthernetPayload {
    IP(IpPacket),
    ICMP,
    ARP,
    Dummy,
}

#[derive(Debug)]
struct VlanTag {
    tpid: [u8; 2],
    pcp: u8,
    dei: u8,
    vid: [u8; 2],
}

#[derive(Component, Debug)]
pub struct EthernetFrame {
    pub dest: MacAddress,
    pub src: MacAddress,
    pub vlan: Option<VlanTag>,
    pub length: [u8; 2],
    pub payload: EthernetPayload,
    pub fcs: [u8; 4],
}

impl EthernetFrame {
    pub fn new(src: MacAddress, dest: MacAddress, payload: EthernetPayload) -> Self {
        Self {
            dest,
            src,
            vlan: None,
            length: [0; 2],
            payload,
            fcs: [0; 4],
        }
    }
}