use bevy::prelude::*;
use bevy::utils::Uuid;

pub struct NetworkDevice {
    id: String,
    name: String,
    device_type: DeviceType,
    status: DeviceStatus,
}

impl NetworkDevice {
    fn new(name: String, device_type: DeviceType) -> Self {
        Self {
            id: Self::generate_id(&device_type),
            name,
            device_type,
            status: DeviceStatus::Active,
        }
    }

    fn generate_id(device_type: &DeviceType) -> String {
        format!("{:?}-{}", device_type, Uuid::new_v4())
    }
}

#[derive(Debug)]
pub enum DeviceType {
    Router,
    Switch,
    Endpoint,
}

// Enumerate possible device statuses
pub enum DeviceStatus {
    Active,
    Inactive,
    Faulty,
}

#[derive(Component)]
pub struct Router {
    pub device: NetworkDevice,
    pub routing_protocol: RoutingProtocol,
}

impl Router {
    pub fn new(name: String, routing_protocol: RoutingProtocol) -> Self {
        Self {
            device: NetworkDevice::new(name, DeviceType::Router),
            routing_protocol,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.device.id
    }

    pub fn get_name(&self) -> &String {
        &self.device.name
    }

    pub fn get_device_type(&self) -> &DeviceType {
        &self.device.device_type
    }

    pub fn get_status(&self) -> &DeviceStatus {
        &self.device.status
    }
}

#[derive(Component)]
pub struct Switch {
    pub device: NetworkDevice,
    pub switch_type: SwitchType,
}

impl Switch {
    pub fn new(name: String, switch_type: SwitchType) -> Self {
        Self {
            device: NetworkDevice::new(name, DeviceType::Switch),
            switch_type,
        }
    }
}
#[derive(Component)]
pub struct Endpoint {
    pub device: NetworkDevice,
    pub os_type: OsType,
}

impl Endpoint {
    pub fn new(name: String, os_type: OsType) -> Self {
        Self {
            device: NetworkDevice::new(name, DeviceType::Endpoint),
            os_type,
        }
    }
}

pub enum RoutingProtocol {
    RIP,
    OSPF,
    BGP,
}

pub enum SwitchType {
    Layer2,
    Layer3,
}

pub enum OsType {
    Windows,
    MacOS,
    Linux,
}
