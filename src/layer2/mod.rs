use bevy::prelude::*;
use systems::{
    peek_queues,
    update_interfaces,
    extract_frames_system,
};

pub mod address;
pub mod pdu;
pub mod interface;
pub mod systems;

pub struct Layer2Plugin;

impl Plugin for Layer2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            (
                peek_queues,
                update_interfaces,
                extract_frames_system,
            ).chain(),
        );
    }
}
