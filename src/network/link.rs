use super::interface::{Direction, Interface};
use bevy::prelude::*;

#[derive(Component)]
pub struct Link(Entity, Entity);

impl Link {
    pub fn new(source: Entity, destination: Entity) -> Self {
        Link(source, destination)
    }

    pub fn transmit_frame(&self) {
        let mut world = World::default();
        match world.get_mut::<Interface>(self.0) {
            Some(mut source) => {
                if let Interface::Ethernet(int) = &mut *source {
                    let frame = int.dequeue_frame(Direction::Out);
                    if let Some(frame) = frame {
                        match world.get_mut::<Interface>(self.1) {
                            Some(mut destination) => {
                                if let Interface::Ethernet(int) = &mut *destination {
                                    int.enqueue_frame(frame, Direction::In);
                                }
                            }
                            None => {
                                println!("Destination interface not found.")
                            }
                        }
                    }
                }
            }
            None => {
                println!("Source interface not found.")
            }
        }
        // let mut source = world.get_mut::<Interface>(self.0).unwrap();
        // if let Interface::Ethernet(int) = &mut *source {
        //     let frame = int.dequeue_frame(Direction::Out);
        //     if let Some(frame) = frame {
        //         let mut destination = world.get_mut::<Interface>(self.1).unwrap();
        //         if let Interface::Ethernet(int) = &mut *destination {
        //             int.enqueue_frame(frame, Direction::In);
        //         }
        //     }
        // }
    }
}
