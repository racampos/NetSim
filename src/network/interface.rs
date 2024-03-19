pub struct Interface {
    pub name: String,
}

impl Interface {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct EthernetInterface {
    pub interface: Interface,
    pub mac_address: String,
    pub speed: u32, // in Mbps
}

impl EthernetInterface {
    pub fn new(name: String, mac_address: String, speed: u32) -> Self {
        Self { 
            interface: Interface::new(name),
            mac_address,
            speed,
        }
    }
}

pub struct FastEthernet {
    pub interface: EthernetInterface,
}

impl FastEthernet {
    pub fn new(name: String, mac_address: String) -> Self {
        Self {
            interface: EthernetInterface::new(name, mac_address, 100),
        }
    }
}

pub struct GigabitEthernet {
    pub interface: EthernetInterface,
}

impl GigabitEthernet {
    pub fn new(name: String, mac_address: String) -> Self {
        Self {
            interface: EthernetInterface::new(name, mac_address, 1000),
        }
    }
}

pub struct Serial {
    pub interface: Interface,
    pub baud_rate: u32,
}

impl Serial {
    pub fn new(name: String, baud_rate: u32) -> Self {
        Self {
            interface: Interface::new(name),
            baud_rate,
        }
    }
}
