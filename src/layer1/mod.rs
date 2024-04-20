use bevy::prelude::*;
use systems::transmit_frames;
use crate::layer2::systems::peek_queues;

pub mod link;
pub mod systems;

pub struct Layer1Plugin;

impl Plugin for Layer1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, transmit_frames.before(peek_queues));
    }
}
