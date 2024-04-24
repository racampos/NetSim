use super::{address::MacAddress, interface::Interface, pdu::EthernetFrame};
use bevy::prelude::*;

pub fn peek_queues(query_interface: Query<(&mut Interface, &Name)>) {
    println!("--------------------------------");
    println!("Time step");
    for (interface, name) in query_interface.iter() {
        if let Interface::Ethernet(int) = interface {
            println!("\n  Peeking queues for interface {:?}", name);
            let frame = int.in_queue.peek();
            match frame {
                Some(f) => println!("    Incoming queue: {}", f),
                None => println!("    Incoming queue: Empty"),
            }
            let frame = int.out_queue.peek();
            match frame {
                Some(f) => println!("    Outgoing queue: {}", f),
                None => println!("    Outgoing queue: Empty"),
            }
        }
    }
}

pub fn update_interfaces(mut query_interface: Query<&mut Interface>) {
    for mut interface in query_interface.iter_mut() {
        if let Interface::Ethernet(int) = &mut *interface {
            // int.short_circuit_queues();
        }
    }
}

pub fn process_frames(mut interfaces: Query<&mut Interface>) {
    for mut interface in interfaces.iter_mut() {
        if let Interface::Ethernet(int) = &mut *interface {
            while !int.in_queue.is_empty() {
                let frame = int.in_queue.dequeue().unwrap();
                if frame.dest == int.mac_address || frame.dest == MacAddress::broadcast() {
                    int.process_frame(&frame);
                    println!("\nARP Table for interface:\n{}", int.arp_table);
                } else {
                    println!(
                        "\nDropping frame with destination {} not matching interface MAC address {}",
                        frame.dest, int.mac_address)
                }
            }
        }
    }
}
