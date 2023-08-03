use bevy::{prelude::{Commands, Handle, Transform, Image, Vec3, Res, AssetServer, default}, sprite::{Sprite, SpriteBundle}};
use rand::random;

fn spawn_tile(commands: &mut Commands, texture: Handle<Image>, x: f32, y:f32, z:f32){
    commands.spawn(SpriteBundle{
        sprite: Sprite::default(),
        texture,          
        transform: Transform { translation: Vec3 { x, y, z }, ..default() },
        ..default()
    });
}

pub fn map(mut commands: Commands, asset_server:Res<AssetServer>){
    for x in -100..=100{
        for y in -100..=100{
            spawn_tile(&mut commands, asset_server.load("grass.png"), (x * 10) as f32, (y * 10) as f32, 0.0);

            let add = random::<f32>();
            if add > 0.95 && add < 0.96
            {
                spawn_tile(&mut commands, asset_server.load("flower.png"), (x * 10) as f32, (y * 10) as f32, 0.1);
            }  
            else if add > 0.997 && add < 0.998
            {
                spawn_tile(&mut commands, asset_server.load("rock.png"), (x * 10) as f32, (y * 10) as f32, 0.1);
            }   
            else if add > 0.998
            {
                spawn_tile(&mut commands, asset_server.load("tree.png"), (x * 10) as f32, (y * 10) as f32, 0.1);
            }   
        } 
    }
}