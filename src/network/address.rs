use regex::Regex;

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
}

pub struct Ipv4Addr {
    pub value: String,
}

impl Ipv4Addr {
    pub fn new(value: String) -> Self {
        let ipv4_address_regex = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
        // TODO: implement a custom error type
        if !ipv4_address_regex.is_match(&value) {
            panic!("Invalid IPv4 address format");
        }
        Self { value }
    }
}

pub struct Ipv6Addr {
    pub value: String,
}

impl Ipv6Addr {
    pub fn new(value: String) -> Self {
        let ipv6_address_regex = Regex::new(r"^(?:[0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}$").unwrap();
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
