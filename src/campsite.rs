use bevy::prelude::*;

use crate::{spawn_builder, spawn_builder2};

#[derive(Event)]
pub struct CreateCampsite;

#[derive(Resource)]
pub struct CampfireState(bool);

fn send_spawn_camp(input: Res<Input<KeyCode>>, mut writer: EventWriter<CreateCampsite>){
    if(input.pressed(KeyCode::C)){
        writer.send(CreateCampsite);
    }
}

fn spawn_camp(mut commands:Commands, mut evt: EventReader<CreateCampsite>, mut campfire_state: ResMut<CampfireState>, asset_server: Res<AssetServer>){
    if(campfire_state.0 == true) { return; }
    for ev in evt.iter() {
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(400.0, 400.0)),
                color: Color::BEIGE,
                ..default()
            },
            transform: Transform{
                translation: Vec3 { z: 1.0, x:530.0, y: 530.0},
                ..default()
            },
            ..default()
        }, Campsite{
            trees: 0
        }));
    
        spawn_builder(&mut commands, &asset_server);
        spawn_builder2(&mut commands, &asset_server);
        
        campfire_state.0 = true;
    }
}

#[derive(Component)]
pub struct Campsite{
    pub trees: i32,
}

pub struct CampsitePlugin;

impl Plugin for CampsitePlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (spawn_tree))
        .insert_resource(CampfireState(false))
        .add_systems(Update, (send_spawn_camp, spawn_camp))
        .add_event::<CreateCampsite>();     
    }
}

fn spawn_campsite(mut commands: Commands){
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(400.0, 400.0)),
            color: Color::BEIGE,
            ..default()
        },
        transform: Transform{
            translation: Vec3 { z: 1.0, x:530.0, y: 530.0},
            ..default()
        },
        ..default()
    }, Campsite{
        trees: 0
    }));
}

#[derive(Component)]
pub struct Tree;

fn spawn_tree(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::BEIGE,
            ..default()
        },
        texture: asset_server.load("tree.png"),
        transform: Transform{
            translation: Vec3 { z: 1.0, x:-330.0, y: 530.0},
            ..default()
        },
        ..default()
    }, Tree));
}