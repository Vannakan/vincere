use std::marker::PhantomData;

use bevy::{prelude::*, window::PrimaryWindow};
use crate::combat::components::{Attacks, Damage, Health, AttackInfo, FindTarget, Attackable};
use crate::common::components::Targetable;
use crate::combat::systems::find_target_with_targetable;
use crate::common::components::BoundingBox;
use crate::game::camera::Minimap;
use crate::game::gold::Inventory;
use crate::ui::components::HasUi;
use crate::ui::events::BindUi;

use crate::common::components::Velocity;
pub mod components;
pub mod attack;
pub mod movement;

use self::attack::*;
use self::movement::*;
use self::components::*;

use super::minion::Minion;

pub struct BanditPlugin;

impl Plugin for BanditPlugin {
    fn build(&self, app: &mut App){
        app     
         .add_systems(Update, add_bandit)
         .add_systems(Update, find_target_with_targetable::<Minion>)
         .add_systems(Update, bandit_found_target)
         .add_systems(Update, move_to_minion)
         .add_systems(Update, attack_minion)
         .add_systems(Update, enemy_defeated)
         .add_systems(Update, default_bandit);
    }
}

fn add_bandit(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    input: Res<Input<KeyCode>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), Without<Minimap>>,
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
   AttackInfo {
    last_attacked: 0.0,
    cooldown: 1.5,
    range: 200.0
    },
    BoundingBox {
        height: 100.0,
        width: 100.0
    },
    FindTarget::<Minion> {
        phantom: PhantomData
    },
    HasUi,
    Inventory{
        coins: 3,
    },
    Targetable,
    Attackable,
    Velocity(Vec3::default()))).id();

    writer.send(BindUi(entity, "Bandit".to_string()));
}
