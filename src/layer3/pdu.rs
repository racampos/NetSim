use bevy::prelude::*;
use crate::layer2::address::MacAddress;
use super::address::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
pub enum Protocols {
    TCP,
    UDP,
}

#[derive(Debug)]
pub struct IpPayload {
    pub data: String,
}

#[derive(Debug)]
pub struct Ipv4Packet {
    pub src: Ipv4Addr,
    pub dest: Ipv4Addr,
    pub tos: u8,
    pub ttl: u8,
    pub protocol: Protocols,
    pub total_length: Option<u16>, // Using Option to represent potentially uninitialized state (similar to None in Python).
    pub payload: IpPayload,
}

impl Ipv4Packet {
    pub fn new(
        src: Ipv4Addr,
        dest: Ipv4Addr,
        payload: IpPayload,
        tos: u8,
        ttl: u8,
        protocol: Protocols,
    ) -> Self {
        Self {
            src,
            dest,
            tos,
            ttl,
            protocol,
            total_length: None,
            payload,
        }
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

#[derive(Debug)]
pub enum IpPacket {
    V4(Ipv4Packet),
    V6(Ipv6Packet),
}

impl IpPacket {
    pub fn new(src: IpAddr, dest: IpAddr, payload: IpPayload, protocol: Protocols) -> Self {
        match src {
            IpAddr::V4(src) => match dest {
                IpAddr::V4(dest) => {
                    IpPacket::V4(Ipv4Packet::new(src, dest, payload, 0, 0, protocol))
                }
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
