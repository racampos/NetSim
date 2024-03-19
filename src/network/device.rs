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

pub trait NetworkDeviceTrait {
    fn get_id(&self) -> &String;
    fn get_name(&self) -> &String;
    fn get_device_type(&self) -> &DeviceType;
    fn get_status(&self) -> &DeviceStatus;
}

#[derive(Component)]
pub struct Router {
    pub base: NetworkDevice,
    pub routing_protocol: RoutingProtocol,
}

impl Router {
    pub fn new(name: String, routing_protocol: RoutingProtocol) -> Self {
        Self {
            base: NetworkDevice::new(name, DeviceType::Router),
            routing_protocol,
        }
    }
}

impl NetworkDeviceTrait for Router {
    fn get_id(&self) -> &String {
        &self.base.id
    }

    fn get_name(&self) -> &String {
        &self.base.name
    }

    fn get_device_type(&self) -> &DeviceType {
        &self.base.device_type
    }

    fn get_status(&self) -> &DeviceStatus {
        &self.base.status
    }
}

#[derive(Component)]
pub struct Switch {
    pub base: NetworkDevice,
    pub switch_type: SwitchType,
}

impl Switch {
    pub fn new(name: String, switch_type: SwitchType) -> Self {
        Self {
            base: NetworkDevice::new(name, DeviceType::Switch),
            switch_type,
        }
    }
}
#[derive(Component)]
pub struct Endpoint {
    pub base: NetworkDevice,
    pub os_type: OsType,
}

impl Endpoint {
    pub fn new(name: String, os_type: OsType) -> Self {
        Self {
            base: NetworkDevice::new(name, DeviceType::Endpoint),
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
