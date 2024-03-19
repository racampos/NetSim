use super::address::{ MacAddress, IpAddr, Ipv4Addr, Ipv6Addr };

pub enum Protocols {
    TCP,
    UDP,
}

pub struct Payload {
    data: String,
}

pub struct Ipv4Packet {
    src: Ipv4Addr,
    dest: Ipv4Addr,
    tos: u8,
    ttl: u8,
    protocol: Protocols,
    total_length: Option<u16>,  // Using Option to represent potentially uninitialized state (similar to None in Python).
}

impl Ipv4Packet {
    fn new(src: Ipv4Addr, dest: Ipv4Addr, payload: Payload, tos: u8, ttl: u8, protocol: Protocols) -> Self {
        Self {
            src,
            dest,
            tos,
            ttl,
            protocol,
            total_length: None,
        }
    }
}

pub struct Ipv6Packet {
    src: Ipv6Addr,
    dest: Ipv6Addr,
    traffic_class: u8,
    hop_limit: u8,
    protocol: Protocols,
    payload_length: Option<u16>,
    flow_label: Option<u32>,
    next_header: Option<u8>,  // Assuming a similar optional field as Python's None.
}

impl Ipv6Packet {
    fn new(src: Ipv6Addr, dest: Ipv6Addr, payload: Payload, traffic_class: u8, hop_limit: u8, protocol: Protocols) -> Self {
        Self {
            src,
            dest,
            traffic_class,
            hop_limit,
            protocol,
            payload_length: None,
            flow_label: None,
            next_header: None,
        }
    }
}

pub enum IpPacket {
    V4(Ipv4Packet),
    V6(Ipv6Packet),
}

impl IpPacket {
    fn new(src: IpAddr, dest: IpAddr, payload: Payload, protocol: Protocols) -> Self {
        match src {
            IpAddr::V4(src) => {
                match dest {
                    IpAddr::V4(dest) => {
                        IpPacket::V4(Ipv4Packet::new(src, dest, payload, 0, 0, protocol))
                    },
                    IpAddr::V6(_) => {
                        panic!("Destination address is not an IPv4 address");
                    },
                }
            },
            IpAddr::V6(src) => {
                match dest {
                    IpAddr::V6(dest) => {
                        IpPacket::V6(Ipv6Packet::new(src, dest, payload, 0, 0, protocol))
                    },
                    IpAddr::V4(_) => {
                        panic!("Destination address is not an IPv6 address");
                    },
                }
            },
        }
    }
}
