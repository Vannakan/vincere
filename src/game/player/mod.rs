pub mod components;
pub mod plugin;
pub mod movement;

use bevy::prelude::*;

use crate::game::gold::Inventory;
use crate::ui::components::HasUi;
use crate::ui::events::BindUi;

use crate::common::components::BoundingBox;
use crate::common::components::Velocity;

use self::components::Player;

const PLAYER_SPEED:f32 = 20.0;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut writer: EventWriter<BindUi>)
{
    let texture = asset_server.load("player.png");
    let entity = commands.spawn((SpriteBundle {
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
    BoundingBox {
        height: 100.0,
        width: 100.0
    },
     Player, 
     Inventory {
        coins: 0 
     }, 
     HasUi,
     Velocity(Vec3::default()))).id();

     writer.send(BindUi(entity, "Player".to_string()));
}


pub fn player_war_horn(mut commands: Commands, input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>){
    if input.just_released(KeyCode::H) {
        commands.spawn(
            AudioBundle{
                source: asset_server.load("audio\\war-horn.wav"),
                settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn, ..Default::default() }
            }
        );
    }
}