use bevy::prelude::*;
use super::{
    interface::Interface,
    pdu::EthernetFrame,
};

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

pub fn extract_frames_system(
    mut interfaces: Query<&mut Interface>,
    frame_query: Query<&EthernetFrame>,
) {
    for mut interface in interfaces.iter_mut() {
        if let Interface::Ethernet(int) = &mut *interface {
            let mut frames = Vec::new();

            while !int.in_queue.is_empty() {
                if let Ok(frame) = frame_query.get(int.in_queue.dequeue().unwrap()) {
                    frames.push(frame);
                } else {
                    println!("Failed to find EthernetFrame for entity {:?}", int.in_queue.dequeue().unwrap());
                }
            }
            // Now `frames` contains all EthernetFrame components from the in_queue
            println!("Extracted frames: {:?}", frames);
        }
    }
}