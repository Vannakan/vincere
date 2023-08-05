use bevy::{prelude::{Commands, Handle, Transform, Image, Vec3, Res, AssetServer, default, Vec2}, sprite::{Sprite, SpriteBundle}};
use rand::{random, Rng};
use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::spawn_tree;

const SMOOTHNESS: f32 = 0.3;
const TILE_SIZE:f32 = 50.0; //50
const MAP_WIDTH:i32 = 50;
const MAP_HEIGHT:i32 = 50;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, map_regen_pressed);
    }
}

#[derive(Component)]
pub struct Tile;

pub fn map_regen_pressed(mut commands: Commands, keycode: Res<Input<KeyCode>>, query:Query<Entity, &Tile>, mut asset_server: ResMut<AssetServer>){

    if keycode.just_released(KeyCode::R){
        regen_map(&mut commands, query, &mut asset_server);
    }
}

fn regen_map(mut commands: &mut Commands, query:Query<Entity, &Tile>, mut asset_server: &mut ResMut<AssetServer>){
    
    for entity in query.iter(){
        commands.entity(entity).despawn();
    }

    map(&mut commands, &mut asset_server)
}

fn spawn_tile(commands: &mut Commands, texture: Handle<Image>, x: f32, y:f32, z:f32){
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        texture,          
        transform: Transform { translation: Vec3 { x, y, z }, ..default() },
        ..default()
    },
    Tile));
}

fn spawn_tile_shallow(commands: &mut Commands, texture: Handle<Image>, x: f32, y:f32, z:f32){
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: Color::TURQUOISE,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        texture,          
        transform: Transform { translation: Vec3 { x, y, z }, ..default() },
        ..default()
    },
    Tile));
}


fn spawn_tile_deep(commands: &mut Commands, texture: Handle<Image>, x: f32, y:f32, z:f32){
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: Color::hex("#0047AB").unwrap(),
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        texture,          
        transform: Transform { translation: Vec3 { x, y, z }, ..default() },
        ..default()
    },
    Tile));
}

pub fn map(mut commands: &mut Commands, asset_server: &mut ResMut<AssetServer>){
    let seed = random::<f32>();
    let mut rand = rand::thread_rng();
    let mut y_off = 0.0;
    for x in -MAP_WIDTH..=MAP_WIDTH {
        let mut x_off = 0.0;
        for y in -MAP_HEIGHT..=MAP_HEIGHT {
            let noise =  simplex_noise_2d_seeded(Vec2 {x :x_off/SMOOTHNESS, y: y_off/SMOOTHNESS }, seed);
            if noise > -0.25 && noise < 0.0
            {
                spawn_tile(&mut commands, asset_server.load("sand.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
            } else if noise > 0.0 {
                spawn_tile(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
                 let add_tree = rand.gen_range(0.0..=1.0);
                 if add_tree >= 0.99
                 {
                    spawn_tree(&mut commands, asset_server, Vec3::from((x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 1.0)))
                 }
            } 
            // Water
            else if noise > -0.75 && noise < -0.50 {
                spawn_tile_shallow(&mut commands, asset_server.load("water.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
            }
            else if noise > -1.0 && noise < -0.75 {
                spawn_tile_deep(&mut commands, asset_server.load("water.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
            }
            else {
                spawn_tile(&mut commands, asset_server.load("water.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
            }
            x_off+=0.01;
        } 
        y_off+=0.01;
    }
}