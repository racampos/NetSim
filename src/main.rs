use bevy::prelude::*;
use netsim::layer1::crc::crc32;
use netsim::layer1::{link::Link, Layer1Plugin};
use netsim::layer2::address::MacAddress;
use netsim::layer2::{
    interface::{
        DestinationInterface, Direction, EthernetInterface, Interface, InterfaceType,
        SourceInterface,
    },
    pdu::{EthernetFrame, EthernetPayload},
    Layer2Plugin,
};

fn make_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let polynomial = 0xedb88320;
    for byte in 0..256 {
        let mut crc = byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ polynomial;
            } else {
                crc >>= 1;
            }
        }
        table[byte] = crc;
    }
    table
}

fn print_crc_table() {
    let table = make_crc_table();
    println!("static CRC_TABLE: [u32; 256] = [");
    for crc in &table {
        print!("0x{:08x}, ", crc);
    }
    println!("];");
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Layer1Plugin, Layer2Plugin))
        .add_systems(
            Startup,
            (setup, add_frame_to_source_interface, connect_interfaces).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let fe_int_1 = EthernetInterface::new(InterfaceType::FastEthernet);
    let fe_int_2 = EthernetInterface::new(InterfaceType::FastEthernet);

    let mac_1 = fe_int_1.mac_address.clone();
    let mac_2 = fe_int_2.mac_address.clone();

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
        EthernetFrame::new(mac_1, MacAddress::broadcast()),
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
