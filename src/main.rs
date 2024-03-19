use bevy::prelude::*;
use netsim::network::device::{Endpoint, OsType, Router, RoutingProtocol, Switch, SwitchType};

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, update_routers)
        .add_systems(Update, update_switches)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Router::new("Router 1".to_string(), RoutingProtocol::OSPF));

    commands.spawn(Switch::new("Switch 1".to_string(), SwitchType::Layer2));

    // Spawn an endpoint entity
    commands.spawn(Endpoint::new("Endpoint 1".to_string(), OsType::Linux));
}

fn update_routers() {
    println!("Updating routers...");
}

fn update_switches() {
    println!("Updating switches...");
}
