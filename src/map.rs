use bevy::{prelude::{Commands, Handle, Transform, Image, Vec3, Res, AssetServer, default, Vec2}, sprite::{Sprite, SpriteBundle}};
use rand::random;
use bevy::prelude::*;
use noisy_bevy::simplex_noise_2d_seeded;

const SMOOTHNESS: f32 = 0.3;
const TILE_SIZE:f32 = 50.0;
const MAP_WIDTH:i32 = 150;
const MAP_HEIGHT:i32 = 150;

#[derive(Component)]
pub struct Tile;

pub fn map_regen_pressed(mut commands: Commands, keycode: Res<Input<KeyCode>>, query:Query<Entity, &Tile>,  asset_server:Res<AssetServer>){

    if(keycode.pressed(KeyCode::R)){
        regen_map(&mut commands, query, asset_server);
    }
}

fn regen_map(mut commands: &mut Commands, query:Query<Entity, &Tile>,  asset_server:Res<AssetServer>){
    
    for entity in query.iter(){
        commands.entity(entity).despawn();
    }

    map(&mut commands, asset_server)
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

pub fn map(mut commands: &mut Commands, asset_server:Res<AssetServer>){
    let seed = random::<f32>();
    
    let mut y_off = 0.0;
    for x in -MAP_WIDTH..=MAP_WIDTH {
        let mut x_off = 0.0;
        for y in -MAP_HEIGHT..=MAP_HEIGHT {
            let noise =  simplex_noise_2d_seeded(Vec2 {x :x_off/SMOOTHNESS, y: y_off/SMOOTHNESS }, seed);
            if noise > 0.0
            {
                spawn_tile(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
            } else {
                spawn_tile(&mut commands, asset_server.load("water.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
            }
            x_off+=0.01;
        } 
        y_off+=0.01;
    }
}