use super::{
    interface::Interface,
    pdu::{EthernetFrame, EthernetPayload},
};
use bevy::{prelude::*, transform::commands};

pub fn peek_queues(query_interface: Query<(&mut Interface, &Name)>) {
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

pub fn update_interfaces(mut query_interface: Query<&mut Interface>) {
    for mut interface in query_interface.iter_mut() {
        if let Interface::Ethernet(int) = &mut *interface {
            int.short_circuit_queues();
        }
    }
}

pub fn process_frames(
    mut commands: Commands,
    mut interfaces: Query<&mut Interface>,
    frame_query: Query<&EthernetFrame>,
) {
    for mut interface in interfaces.iter_mut() {
        if let Interface::Ethernet(int) = &mut *interface {
            while !int.in_queue.is_empty() {
                let frame_entity = int.in_queue.dequeue().unwrap();
                if let Ok(frame) = frame_query.get(frame_entity) {
                    if frame.dest == int.mac_address {
                        commands.entity(frame_entity).despawn();
                        match &frame.payload {
                            EthernetPayload::Dummy => {
                                println!("Received dummy frame");
                            }
                            EthernetPayload::ARP(_arp) => {
                                println!("Received ARP frame");
                            }
                            EthernetPayload::ICMP => {
                                println!("Received ICMP frame");
                            }
                            EthernetPayload::IP(ip_packet) => {
                                println!("Received IP frame: {:?}", ip_packet);
                            }
                            _ => {
                                println!("Received frame with unknown payload");
                            }
                        }
                    }
                } else {
                    println!(
                        "Failed to find EthernetFrame for entity {:?}",
                        int.in_queue.dequeue().unwrap()
                    );
                }
            }
        }
    }
}
