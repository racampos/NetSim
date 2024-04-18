use bevy::prelude::*;
use netsim::network::address::{IpAddr, Ipv4Addr, Ipv6Addr, MacAddress};
use netsim::network::device::{Endpoint, OsType, Router, RouterModel, Switch, SwitchModel};
use netsim::network::interface::{
    DestinationInterface, Direction, EthernetInterface, Interface, InterfaceType, SerialInterface,
    SourceInterface,
};
use netsim::network::link::Link;
use netsim::network::pdu::{EthernetFrame, EthernetPayload};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup,
                add_frame_to_interface.after(setup),
                connect_interfaces.after(add_frame_to_interface),
            ),
        )
        .add_systems(Update, transmit_frames)
        // .add_systems(Update, update_routers)
        // .add_systems(Update, update_interfaces)
        .add_systems(Update, update_frames)
        .run();
}

fn setup(mut commands: Commands) {
    // // Create a FastEthernet interface
    // let mut fe_int = EthernetInterface::new(InterfaceType::FastEthernet);
    // fe_int.set_ipv4_address(Ipv4Addr::new("192.168.100.1".to_string()));
    // fe_int.add_ipv6_address(Ipv6Addr::new("2001:db8:123:456::1".to_string()));

    // // Spawn a router entity
    // let router_id = commands
    //     .spawn((Router::new(RouterModel::Generic), Name::new("R1")))
    //     .id();

    let mut fe_int_1 = EthernetInterface::new(InterfaceType::FastEthernet);
    let mut fe_int_2 = EthernetInterface::new(InterfaceType::FastEthernet);

    let mac_1 = fe_int_1.mac_address.clone();
    let mac_2 = fe_int_2.mac_address.clone();

    // Spawn the interface entities
    let int1 = commands
        .spawn((
            Interface::Ethernet(fe_int_1),
            Name::new("FE1"),
            SourceInterface,
        ))
        .id();

    let int2 = commands
        .spawn((
            Interface::Ethernet(fe_int_2),
            Name::new("FE2"),
            DestinationInterface,
        ))
        .id();

    let frame = commands.spawn((
        EthernetFrame {
            src: mac_1,
            dest: mac_2,
            payload: EthernetPayload::Dummy,
        },
        Name::new("ARP Frame"),
    ));
}

fn update_routers(query: Query<(&Router, &Name)>) {
    for (router, name) in &query {
        println!("Router {} is of model {:?}.", name, router.model);
    }
}

fn update_interfaces(query: Query<(&Interface, &Name)>) {
    for (interface, name) in &query {
        println!(
            "Interface {} is of type {:?}.",
            name,
            if let Interface::Ethernet(e) = interface {
                &e.interface_type
            } else {
                &InterfaceType::FastEthernet
            }
        );
    }
}

fn add_frame_to_interface(
    query_frame: Query<(Entity, &EthernetFrame)>,
    mut query_interface: Query<&mut Interface, With<SourceInterface>>,
) {
    let (frame_entity, _frame) = query_frame.single();
    let mut interface = query_interface.single_mut();
    println!("Original frame: {:?}", frame_entity);

    if let Interface::Ethernet(int) = &mut *interface {
        int.enqueue_frame(frame_entity, Direction::Out);
    }
}

fn connect_interfaces(
    mut commands: Commands,
    query_source: Query<Entity, With<SourceInterface>>,
    query_dest: Query<Entity, With<DestinationInterface>>,
) {
    let source_entity = query_source.single();
    let dest_entity = query_dest.single();

    commands.spawn(Link::new(source_entity, dest_entity));
}

fn transmit_frames(mut query_link: Query<&Link>) {
    let link = query_link.single();
    link.transmit_frame();
}

fn update_frames(mut query_interface: Query<&mut Interface, With<SourceInterface>>) {
    let interface = query_interface.single();

    if let Interface::Ethernet(int) = interface {
        let frame = int.out_queue.peek();
        match frame {
            Some(f) => println!("Peeked frame: {:?}", f),
            None => println!("No frames in the queue"),
        }
    }
}
