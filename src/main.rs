use bevy::prelude::*;
use netsim::network::address::{IpAddr, Ipv4Addr, Ipv6Addr, MacAddress};
use netsim::network::device::{Endpoint, OsType, Router, RouterModel, Switch, SwitchModel};
use netsim::network::interface::{EthernetInterface, Interface, InterfaceType, SerialInterface};

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, update_routers)
        .add_systems(Update, update_switches)
        .run();
}

fn setup(mut commands: Commands) {
    // Create a FastEthernet interface
    let mut fe_int =
        EthernetInterface::new("FastEthernet0/0".to_string(), InterfaceType::FastEthernet);
        fe_int.set_ipv4_address(Ipv4Addr::new("192.168.100.1".to_string()));
        fe_int.add_ipv6_address(Ipv6Addr::new("2001:db8:123:456::1".to_string()));

    // Spawn the interface entity
    let int_id = commands.spawn((
        Interface::Ethernet(fe_int),
        Name::new("FastEthernet0/0"),
    )).id();

    // Spawn the router entity
    let router_id = commands.spawn((
        Router::new(RouterModel::Generic), 
        Name::new("R1",
    ))).id();

    if let Ok(mut router) = commands.entity(router_id).get_mut::<Router>() {
        router.add_interface(int_id);
    }

    if let Ok(mut interface) = commands.entity(int_id).get_mut::<Interface>() {
        interface.attach_to_device(Some(router_id));
    }


    commands.spawn(Switch::new(SwitchModel::Generic));

    commands.spawn(Endpoint::new(OsType::Linux));
}

fn update_routers(query: Query<(&Router, &Name)>) {
    for (router, name) in &query {
        println!("Router {} is of model {:?}.", name, router.model);
    }
}

fn update_switches() {
    println!("Updating switches...");
}
