use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, character_movement);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    let texture = asset_server.load("player.png");
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3 { z: 1.0, ..default()},
            ..default()
        },
        ..default()
    }, Player));
}

#[derive(Bundle)]
pub struct PlayerBundle {
    sprit_bundle: SpriteBundle,
    player: Player
}

#[derive(Component)]
pub struct Player;

pub fn character_movement (
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut player = player_query.single_mut();
    if input.pressed(KeyCode::D) {
        player.translation.x += 150.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        player.translation.x -= 150.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::W) {
        player.translation.y += 150.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        player.translation.y -= 150.0 * time.delta_seconds();
    }

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}


// mut camera: Query<(&mut Transform, &mut Camera), Without<Sprite>>,

// let Ok((mut camera_transform, _)) = camera.get_single_mut() else { return };


// camera_transform.translation.y = transform.translation.y;
// camera_transform.translation.x = transform.translation.x;