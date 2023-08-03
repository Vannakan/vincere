use bevy::prelude::*;

const PLAYER_SPEED:f32 = 20.0;

pub struct PlayerPlugin;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprit_bundle: SpriteBundle,
    player: Player,
    velocity: Velocity
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec3);


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, (player_movement, player_input, camera_movement));
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
    },
     Player, 
     Velocity(Vec3::default())));
}

pub fn player_movement(mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>, time: Res<Time>,){
    let (mut transform, mut velocity) = player_query.single_mut();

    transform.translation += velocity.0 * time.delta_seconds();

    if velocity.0.x >= -0.1 && velocity.0.x <= 0.1 && velocity.0.y <= 0.1 && velocity.0.y >= -0.1
    {
        velocity.0 = Vec3::default();
    }
    else {
        velocity.0 = velocity.0.lerp(Vec3::default(), 0.1)
    }
}

pub fn camera_movement(mut player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>){
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