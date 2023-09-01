use bevy::prelude::*;

use crate::{common::components::Velocity, game::camera::Minimap};

use super::{components::Player, PLAYER_SPEED};

pub fn camera_movement(mut player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>, Without<Minimap>)>)
    {
        let mut camera = camera_query.single_mut();
        let player = player_query.single_mut();

        let direction = camera.translation.lerp(player.translation, 0.05);
        
        camera.translation.x = direction.x;
        camera.translation.y = direction.y;
    }

pub fn player_input (
    mut player_query: Query<&mut Velocity, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let mut player = player_query.single_mut();
    if input.pressed(KeyCode::D) {
        player.0 += Vec3{x: PLAYER_SPEED, y:0.0, z:0.0}
    }
    if input.pressed(KeyCode::A) {
        player.0 -= Vec3{x: PLAYER_SPEED, y:0.0, z:0.0}
    }
    if input.pressed(KeyCode::W) {
        player.0 += Vec3{x: 0.0, y:PLAYER_SPEED, z:0.0}
    }
    if input.pressed(KeyCode::S) {
        player.0 -= Vec3{x: 0.0, y:PLAYER_SPEED, z:0.0}
    }
}