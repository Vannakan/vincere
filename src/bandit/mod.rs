use bevy::{prelude::*, window::PrimaryWindow};
use crate::{ Velocity, Damage, Health, HasUi, BindUi, Attacks};
pub mod components;
mod attack;
mod movement;

use crate::bandit::attack::*;
use crate::bandit::movement::*;
use crate::bandit::components::*;
pub struct BanditPlugin;

impl Plugin for BanditPlugin {
    fn build(&self, app: &mut App){
        app     
         .add_systems(Update, add_bandit)
         .add_systems(Update, check_minion_range)
         .add_systems(Update, move_to_minion)
         .add_systems(Update, attack_minion);
    }
}


fn add_bandit(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    input: Res<Input<KeyCode>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    writer: EventWriter<BindUi>)
    {
        if let Some(position) = q_windows.single().cursor_position(){
        let (camera, camera_transform) = camera_q.single();
        if input.just_released(KeyCode::B)
        {
            spawn_bandit(&mut commands, &mut asset_server, Vec3::from((camera.viewport_to_world_2d(camera_transform, position).unwrap(), 1.0)), writer)
        }
    }
}

fn spawn_bandit(commands: &mut Commands, asset_server: &mut ResMut<AssetServer>, position: Vec3, mut writer: EventWriter<BindUi>){
    let texture = asset_server.load("bandit.png");
    let entity =  commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: position,
            ..default()
        },
        ..default()
    }, 
    Idle,
    Bandit,
    Health{
        starting: 10.0,
        current: 10.0
    },
    Damage(3.0),
    Attacks {
        last_attacked: 0.0
   },
    HasUi,
    Velocity(Vec3::default()))).id();

    writer.send(BindUi(entity, "Bandit".to_string()));
}
