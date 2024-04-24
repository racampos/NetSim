use bevy::prelude::*;
use netsim::layer1::crc::crc32;
use netsim::layer1::{link::Link, hub::Hub, Layer1Plugin};
use netsim::layer2::address::MacAddress;
use netsim::layer2::{
    interface::{
        DestinationInterface, Direction, EthernetInterface, Interface, InterfaceType,
        SourceInterface,
    },
    pdu::{EthernetFrame, EthernetPayload},
    Layer2Plugin,
};
use netsim::layer3::address::Ipv4Addr;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(1.0))
        .add_plugins((DefaultPlugins, Layer1Plugin, Layer2Plugin))
        .add_systems(
            Startup,
            (setup, add_frame_to_source_interface, connect_interfaces).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut fe_int_1 = EthernetInterface::new(InterfaceType::FastEthernet);
    fe_int_1.set_ipv4_address(Ipv4Addr::new("192.168.1.1"));
    let mut fe_int_2 = EthernetInterface::new(InterfaceType::FastEthernet);
    fe_int_2.set_ipv4_address(Ipv4Addr::new("192.168.1.2"));
    let mut fe_int_3 = EthernetInterface::new(InterfaceType::FastEthernet);
    fe_int_3.set_ipv4_address(Ipv4Addr::new("192.168.1.3"));

    let mac_1 = fe_int_1.mac_address.clone();
    let mac_2 = fe_int_2.mac_address.clone();
    let mac_3 = fe_int_3.mac_address.clone();

    // Spawn the interface entities
    commands.spawn((
        Interface::Ethernet(fe_int_1),
        Name::new("FE1"),
        SourceInterface,
    ));

    commands.spawn((
        Interface::Ethernet(fe_int_2),
        Name::new("FE2"),
        DestinationInterface,
    ));

    commands.spawn((
        Interface::Ethernet(fe_int_3),
        Name::new("FE3"),
        DestinationInterface,
    ));
}

fn add_frame_to_source_interface(
    mut query_interface: Query<&mut Interface, With<SourceInterface>>,
) {
    let mut interface = query_interface.single_mut();
    if let Interface::Ethernet(int) = &mut *interface {
        int.send_arp_request(Ipv4Addr::new("192.168.1.2"));
    }
}

fn connect_interfaces(
    mut commands: Commands,
    query_interface: Query<(Entity, &Interface)>
) {
    let mut vec_entities: Vec<Entity> = Vec::new();
    for (interface_entity, _) in query_interface.iter() {
        vec_entities.push(interface_entity);
    }

    commands.spawn((Hub::new(vec_entities), Name::new("Hub")));
}
