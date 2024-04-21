use crate::layer2::systems::peek_queues;
use bevy::prelude::*;
use systems::transmit_frames;

pub mod crc;
pub mod link;
pub mod systems;

pub struct Layer1Plugin;

impl Plugin for Layer1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, transmit_frames.before(peek_queues));
    }
}
