use bevy::prelude::*;
use vincere::{hello::HelloPlugin, startup::{build_default_plugins, setup}, player::character_movement};

fn main() {
    App::new()
    .add_plugins((build_default_plugins(), HelloPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, character_movement)
    .run();
}
