use bevy::prelude::*;
use crate::common::components::Velocity;

use super::movement::camera_movement;
use super::movement::player_input;
use super::player_war_horn;
use super::{spawn_player, components::Player};

pub struct PlayerPlugin;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprit_bundle: SpriteBundle,
    player: Player,
    velocity: Velocity
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, (player_input, camera_movement,player_war_horn));
    }
}

