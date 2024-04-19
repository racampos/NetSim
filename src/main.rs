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
                add_frame_to_source_interface.after(setup),
                connect_interfaces.after(add_frame_to_source_interface),
            ),
        )
        .add_systems(
            Update,
            (
                transmit_frames,
                peek_queues_1.after(transmit_frames),
                update_interfaces.after(peek_queues_1),
                peek_queues_2.after(update_interfaces),
            ),
        )
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
        EthernetFrame::new(mac_1, mac_2, EthernetPayload::Dummy),
        Name::new("ARP Frame"),
    ));
}

fn add_frame_to_source_interface(
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

fn transmit_frames(links: Query<&Link>, mut interfaces: Query<&mut Interface>) {
    for link in links.iter() {
        // Transmit frame from link.0 to link.1
        Link::transmit_frame(link.0, link.1, &mut interfaces);

        // Transmit frame from link.1 to link.0
        Link::transmit_frame(link.1, link.0, &mut interfaces);
    }
}

fn peek_queues_1(query_interface: Query<(&mut Interface, &Name)>) {
    println!("--------------------------------");
    println!("Time step");
    for (interface, name) in query_interface.iter() {
        if let Interface::Ethernet(int) = interface {
            println!("  Peeking queues for interface {:?}", name);
            let frame = int.in_queue.peek();
            match frame {
                Some(f) => println!("    Incoming queue: {:?}", f),
                None => println!("    Incoming queue: Empty"),
            }
            let frame = int.out_queue.peek();
            match frame {
                Some(f) => println!("    Outgoing queue: {:?}", f),
                None => println!("    Outgoing queue: Empty"),
            }
        }
    }
}

fn peek_queues_2(query_interface: Query<(&mut Interface, &Name)>) {
    println!("--------------------------------");
    println!("Time step");
    for (interface, name) in query_interface.iter() {
        if let Interface::Ethernet(int) = interface {
            println!("  Peeking queues for interface {:?}", name);
            let frame = int.in_queue.peek();
            match frame {
                Some(f) => println!("    Incoming queue: {:?}", f),
                None => println!("    Incoming queue: Empty"),
            }
            let frame = int.out_queue.peek();
            match frame {
                Some(f) => println!("    Outgoing queue: {:?}", f),
                None => println!("    Outgoing queue: Empty"),
            }
        }
    }
}

fn update_interfaces(mut query_interface: Query<(&mut Interface, &Name)>) {
    for (mut interface, name) in query_interface.iter_mut() {
        if let Interface::Ethernet(int) = &mut *interface {
            int.short_circuit_queues();
        }
    }
}
