use super::super::layer2::interface::{Direction, Interface};
use bevy::prelude::*;

#[derive(Component)]
pub struct Hub {
    pub interfaces: Vec<Entity>,
}

impl Hub {
    pub fn new(interfaces: Vec<Entity>) -> Self {
        Hub { interfaces }
    }

    pub fn transmit_frame(&self, interfaces: &mut Query<&mut Interface>) {
        for interface in self.interfaces.iter() {
            match interfaces.get_mut(*interface) {
                Ok(mut eth_interface) => {
                    if let Interface::Ethernet(eth) = &mut *eth_interface {
                        if let Some(frame) = eth.dequeue_frame(Direction::Out) {
                            for dest_interface in self.interfaces.iter() {
                                if dest_interface != interface {
                                    match interfaces.get_mut(*dest_interface) {
                                        Ok(mut dest_eth_interface) => {
                                            if let Interface::Ethernet(dest_eth) =
                                                &mut *dest_eth_interface
                                            {
                                                dest_eth.enqueue_frame(frame.clone(), Direction::In);
                                            }
                                        }
                                        Err(_) => println!("Destination interface not found."),
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => println!("Source interface not found."),
            }
        }
    }
}