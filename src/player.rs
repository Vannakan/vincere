use bevy::prelude::*;

pub fn character_movement (mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {

    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
    }
}