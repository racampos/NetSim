use bevy::prelude::*;
use netsim::network::address::{IpAddr, Ipv4Addr, Ipv6Addr, MacAddress};
use netsim::network::device::{Endpoint, OsType, Router, RouterModel, Switch, SwitchModel};
use netsim::network::interface::{EthernetInterface, Interface, InterfaceType, SerialInterface};
use netsim::network::utils::Name;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, update_routers)
        .add_systems(Update, update_switches)
        .run();
}

fn setup(mut commands: Commands) {
    // Create a router
    let mut router = Router::new(RouterModel::Generic);
    // Create a FastEthernet interface
    let mut interface =
        EthernetInterface::new("FastEthernet0/0".to_string(), InterfaceType::FastEthernet);
    interface.set_ipv4_address(Ipv4Addr::new("192.168.100.1".to_string()));
    interface.add_ipv6_address(Ipv6Addr::new("2001:db8:123:456::1".to_string()));

    commands.spawn((
        Interface::Ethernet(interface),
        Name("FastEthernet0/0".to_string()),
    ));

    // Add the interface to the router
    router.add_interface(Interface::Ethernet(interface));
    // Spawn the router entity
    commands.spawn((router, Name("R1".to_string())));

    commands.spawn(Switch::new(SwitchModel::Generic));

    commands.spawn(Endpoint::new(OsType::Linux));
}

fn update_routers() {
    println!("Updating routers...");
}

fn update_switches() {
    println!("Updating switches...");
}
