use regex::Regex;
use uuid::Uuid;

pub struct MacAddress {
    pub address: String,
}

impl MacAddress {
    pub fn new(address: String) -> Self {
        let mac_address_regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
        // TODO: implement a custom error type
        if !mac_address_regex.is_match(&address) {
            panic!("Invalid MAC address format");
        }
        Self { address }
    }

    pub fn random() -> Self {
        // Generate a random UUID
        let uuid = Uuid::new_v4();

        // Get the simple format which omits hyphens
        let simple = uuid.simple().to_string();

        // Take the first 12 characters (6 bytes) to simulate a MAC address
        // MAC addresses are typically represented in hexadecimal format, separated by colons
        let mut mac_parts = Vec::new();
        for i in (0..12).step_by(2) {
            mac_parts.push(&simple[i..i + 2]);
        }

        // Join the parts using colons to format it as a typical MAC address
        Self::new(mac_parts.join(":").to_uppercase())
    }
}

pub struct Ipv4Addr {
    pub octets: Vec<u8>,
}

impl Ipv4Addr {
    pub fn new(value: String) -> Self {
        let ipv4_address_regex = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
        // TODO: implement a custom error type
        if !ipv4_address_regex.is_match(&value) {
            panic!("Invalid IPv4 address format");
        }
        let octets: Vec<u8> = value
            .split(".")
            .map(|octet| octet.parse().unwrap())
            .collect();
        Self { octets }
    }

    pub fn to_string(&self) -> String {
        self.octets.iter().map(|octet| octet.to_string()).collect::<Vec<String>>().join(".")
    }

    pub fn get_network_address(&self, subnet_mask: &Ipv4Addr) -> Ipv4Addr {
        let network_octets: Vec<u8> = self
            .octets
            .iter()
            .zip(subnet_mask.octets.iter())
            .map(|(octet, mask_octet)| octet & mask_octet)
            .collect();
        Ipv4Addr {
            octets: network_octets,
        }
    }
}

pub struct Ipv6Addr {
    pub value: String,
}

impl Ipv6Addr {
    pub fn new(value: String) -> Self {
        // TODO: Consider using Rust's built-in Ipv6Addr type
        // use std::net::Ipv6Addr;
        // Something like this:
        // fn validate_ipv6_address(address: &str) -> bool {
        //    address.parse::<Ipv6Addr>().is_ok()
        // }
        let ipv6_address_regex = Regex::new(r"^(?:[0-9A-Fa-f]{1,4}:){7}(?:[0-9A-Fa-f]{1,4}|:)|(?:[0-9A-Fa-f]{1,4}:){6}(?::[0-9A-Fa-f]{1,4}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){5}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,2}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){4}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,3}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){3}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,4}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){2}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,5}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:)(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,6}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:::(?:[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,7}|[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:))$
        ").unwrap();
        // TODO: implement a custom error type
        if !ipv6_address_regex.is_match(&value) {
            panic!("Invalid IPv6 address format");
        }
        Self { value }
    }
}

pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    pub fn new(value: String) -> Self {
        if value.contains(":") {
            IpAddr::V6(Ipv6Addr::new(value))
        } else {
            IpAddr::V4(Ipv4Addr::new(value))
        }
    }
}
