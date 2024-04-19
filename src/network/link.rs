use super::interface::{Direction, Interface};
use bevy::prelude::*;

#[derive(Component)]
pub struct Link(pub Entity, pub Entity);

impl Link {
    pub fn new(source: Entity, destination: Entity) -> Self {
        Link(source, destination)
    }

    pub fn transmit_frame(
        source: Entity,
        destination: Entity,
        interfaces: &mut Query<&mut Interface>,
    ) {
        match interfaces.get_mut(source) {
            Ok(mut src_interface) => {
                if let Interface::Ethernet(src_eth_interface) = &mut *src_interface {
                    if let Some(frame) = src_eth_interface.dequeue_frame(Direction::Out) {
                        match interfaces.get_mut(destination) {
                            Ok(mut dest_interface) => {
                                if let Interface::Ethernet(dest_eth_interface) =
                                    &mut *dest_interface
                                {
                                    dest_eth_interface.enqueue_frame(frame, Direction::In);
                                }
                            }
                            Err(_) => println!("Destination interface not found."),
                        }
                    }
                }
            }
            Err(_) => println!("Source interface not found."),
        }
    }
}
