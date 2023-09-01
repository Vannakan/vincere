use bevy::prelude::*;
use rand::Rng;

use crate::common::components::Velocity;

#[derive(Component)]
pub struct Rabbit{
    pub next_move: f32
}

pub struct RabbitPlugin;

impl Plugin for RabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rabbit_movement);
    }
}

pub fn spawn_rabbit(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let texture = asset_server.load("rabbit.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3 { z: 1.0, ..default()},
            ..default()
        },
        ..default()
    },
    // BoundingBox {
    //     height: 100.0,
    //     width: 100.0
    // },
    Rabbit {
        next_move: 0.0
    },
     Velocity(Vec3::default()))).id();
}

pub fn spawn_rabbit_2(commands: &mut Commands, asset_server: &Res<AssetServer>, x:f32, y:f32){
    let mut rng = rand::thread_rng();
    let texture = asset_server.load("rabbit.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3 { z: 1.0, x,y},
            ..default()
        },
        ..default()
    },
    // BoundingBox {
    //     height: 100.0,
    //     width: 100.0
    // },
    Rabbit {
        next_move: 0.0
    },
     Velocity(Vec3::default()))).id();
}


pub fn rabbit_movement(mut query: Query<(Entity, &mut Velocity, &mut Rabbit)>, time: Res<Time>) {
    if query.is_empty() { return; } 

    let mut rng = rand::thread_rng();
    
    for (entity, mut velocity, mut rabbit) in query.iter_mut(){
        if rabbit.next_move <= time.elapsed_seconds() {
            println!("hello");
            let x = rng.gen_range(-1..=1) * 250;
            let y = rng.gen_range(-1..=1) * 250;

            velocity.0 += Vec3{x: x as f32, y: y as f32, z: 0.0};
            rabbit.next_move = time.elapsed_seconds() + rng.gen_range(3..=6) as f32;
        }
    }
}