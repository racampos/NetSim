use bevy::prelude::*;

#[derive(Component)]
pub struct Link {
    pub interfaces: Vec<Entity>,
}
