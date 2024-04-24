use regex::Regex;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Ipv4Addr {
    pub octets: [u8; 4],
}

impl Ipv4Addr {
    pub fn new(value: &str) -> Self {
        let value = value.to_string();
        let ipv4_address_regex = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
        // TODO: implement a custom error type
        if !ipv4_address_regex.is_match(&value) {
            panic!("Invalid IPv4 address format");
        }
        let octets: Vec<u8> = value
            .split(".")
            .map(|octet| octet.parse::<u8>().unwrap())
            .collect();
        Self {
            octets: [octets[0], octets[1], octets[2], octets[3]],
        }
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
            octets: [
                network_octets[0],
                network_octets[1],
                network_octets[2],
                network_octets[3],
            ],
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let [first, second, third, fourth] = self.octets;
        [first, second, third, fourth]
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.octets[0], self.octets[1], self.octets[2], self.octets[3]
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ipv6Addr {
    pub value: String,
}

impl Ipv6Addr {
    pub fn new(value: &str) -> Self {
        // TODO: Consider using Rust's built-in Ipv6Addr type
        // use std::net::Ipv6Addr;
        // Something like this:
        // fn validate_ipv6_address(address: &str) -> bool {
        //    address.parse::<Ipv6Addr>().is_ok()
        // }
        let value = value.to_string();
        let ipv6_address_regex = Regex::new(r"^(?:[0-9A-Fa-f]{1,4}:){7}(?:[0-9A-Fa-f]{1,4}|:)|(?:[0-9A-Fa-f]{1,4}:){6}(?::[0-9A-Fa-f]{1,4}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){5}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,2}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){4}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,3}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){3}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,4}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:){2}(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,5}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:[0-9A-Fa-f]{1,4}:)(?::[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,6}|:[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:)|(?:::(?:[0-9A-Fa-f]{1,4}|(?::[0-9A-Fa-f]{1,4}){1,7}|[0-9A-Fa-f]{1,4}:\d+\.\d+\.\d+\.\d+|:))$
        ").unwrap();
        // TODO: implement a custom error type
        if !ipv6_address_regex.is_match(&value) {
            panic!("Invalid IPv6 address format");
        }
        Self { value }
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    pub fn new(value: &str) -> Self {
        if value.contains(":") {
            IpAddr::V6(Ipv6Addr::new(value))
        } else {
            IpAddr::V4(Ipv4Addr::new(value))
        }
    }
}
