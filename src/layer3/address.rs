use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
        self.octets
            .iter()
            .map(|octet| octet.to_string())
            .collect::<Vec<String>>()
            .join(".")
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
