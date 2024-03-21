use super::address::{IpAddr, MacAddress};

pub enum Speed {
    TenMbps,
    HundredMbps,
    OneGbps,
}

pub enum Interface {
    FastEthernet(FastEthernet),
    GigabitEthernet(GigabitEthernet),
    Serial(Serial),
}

pub struct FastEthernet {
    pub name: String,
    pub mac_address: MacAddress,
    pub ip_addresses: Vec<IpAddr>,
    pub speed: Speed,
}

impl FastEthernet {
    pub fn new(name: String, mac_address: MacAddress, ip_addresses: Vec<IpAddr>) -> Self {
        Self {
            name,
            mac_address,
            ip_addresses,
            speed: Speed::HundredMbps,
        }
    }
}

pub struct GigabitEthernet {
    pub mac_address: MacAddress,
    pub ip_addresses: Vec<IpAddr>,
    pub speed: Speed,
}

impl GigabitEthernet {
    pub fn new(mac_address: MacAddress, ip_addresses: Vec<IpAddr>) -> Self {
        Self {
            mac_address,
            ip_addresses,
            speed: Speed::OneGbps,
        }
    }
}

pub struct Serial {
    pub ip_addresses: Vec<IpAddr>,
}

impl Serial {
    pub fn new(ip_addresses: Vec<IpAddr>) -> Self {
        Self { ip_addresses }
    }
}
