use super::address::{IpAddr, Ipv4Addr, Ipv6Addr, MacAddress};
use bevy::prelude::*;
use std::collections::HashMap;

pub enum InterfaceType {
    FastEthernet,
    GigabitEthernet,
}

#[derive(Component)]
pub enum Interface {
    Ethernet(EthernetInterface),
    Serial(SerialInterface),
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

pub struct EthernetInterface {
    pub interface_type: InterfaceType,
    pub name: String,
    pub mac_address: MacAddress,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_addresses: Vec<Ipv6Addr>,
    pub arp_table: ArpTable,
}

impl EthernetInterface {
    pub fn new(name: String, interface_type: InterfaceType) -> Self {
        Self {
            name,
            interface_type: interface_type,
            mac_address: MacAddress::random(),
            ipv4_address: None,
            ipv6_addresses: Vec::new(),
            arp_table: ArpTable::new(),
        }
    }

    pub fn set_ipv4_address(&mut self, ipv4_address: Ipv4Addr) {
        self.ipv4_address = Some(ipv4_address);
    }

    pub fn add_ipv6_address(&mut self, ipv6_address: Ipv6Addr) {
        self.ipv6_addresses.push(ipv6_address);
    }
}

pub struct SerialInterface {
    pub ip_addresses: Vec<IpAddr>,
}

impl SerialInterface {
    pub fn new(ip_addresses: Vec<IpAddr>) -> Self {
        Self { ip_addresses }
    }
}
