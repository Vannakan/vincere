// Each camp spawns peasants randomly

use bevy::prelude::*;
use rand::Rng;

use crate::{spawn_peasant2, ui::events::BindUi};

pub struct CampPlugin;

impl Plugin for CampPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camp);
    }
}


#[derive(Component)]
pub struct Camp;

pub fn spawn_camp_2(mut commands: &mut Commands, asset_server: &Res<AssetServer>, mut writer: &mut EventWriter<BindUi>, pos: Vec2){
    let texture = asset_server.load("fire.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3::from([pos.x, pos.y, 1.0]),
            ..default()
        },
        ..default()
    }, 
    Camp)).id();

    let mut rng = rand::thread_rng();
    for _ in 0..=1{
        let x_offset = rng.gen_range(-100.0..100.0);
        let y_offset = rng.gen_range(-100.0..100.0);

        spawn_peasant2(&mut commands, &asset_server, &mut writer, Vec2::from([pos.x + x_offset, pos.y + y_offset]))
    }

}



pub fn spawn_camp(mut commands: Commands, asset_server: Res<AssetServer>, mut writer: EventWriter<BindUi>){
    let texture = asset_server.load("fire.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3::from([-100.0, -100.0, 1.0]),
            ..default()
        },
        ..default()
    }, 
    Camp)).id();

    let mut rng = rand::thread_rng();
    for _ in 0..=1{
        let x_offset = rng.gen_range(-100.0..100.0);
        let y_offset = rng.gen_range(-100.0..100.0);

        spawn_peasant2(&mut commands, &asset_server, &mut writer, Vec2::from([-100.0 + x_offset, -100.0 + y_offset]))
    }

}
