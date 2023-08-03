use std::borrow::BorrowMut;
use bevy::prelude::*;

use crate::{player::Player, Velocity};

#[derive(Component)]
pub struct Minion;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_minion)      
         .add_systems(Update, minion_follow_player)
         .add_systems(Update, minion_avoidance)
         .add_systems(Update, minion_movement);
    }
}

fn spawn_minion(mut commands: Commands, asset_server: Res<AssetServer>){
    let texture = asset_server.load("minion.png");
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3 { z: 1.0, x: 50.0, y: 50.0},
            ..default()
        },
        ..default()
    }, 
    Minion,
    Velocity(Vec3::default())));

    let texture = asset_server.load("minion.png");
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3 { z: 1.0, x: 50.0, y: 150.0},
            ..default()
        },
        ..default()
    }, Minion,
    Velocity(Vec3::default())));
}


pub fn minion_movement(
    mut minion_query: Query<(&mut Transform, &mut Velocity),(With<Minion>, Without<Player>)>, time: Res<Time>,){
    for minion in minion_query.iter_mut(){
        let (mut transform, mut velocity) = minion;

        transform.translation += velocity.0 * time.delta_seconds();
    
        if velocity.0.x >= -0.1 && velocity.0.x <= 0.1 && velocity.0.y <= 0.1 && velocity.0.y >= -0.1
        {
            velocity.0 = Vec3::default();
        }
        else {
            velocity.0 = velocity.0.lerp(Vec3::default(), 0.1)
        }
    }
}

pub fn minion_avoidance(mut minion_query: Query<(&mut Transform, &mut Velocity), (With<Minion>, Without<Player>)>){
    let mut combinations = minion_query.iter_combinations_mut();
    while let Some([mut t1, t2]) = combinations.fetch_next(){
        if t1.0.translation.distance(t2.0.translation) > 50.0 { return;}

        let direction = (t2.0.translation - t1.0.translation).normalize();
        t1.borrow_mut().1.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}
    }
}

pub fn minion_follow_player(
    mut minion_query: Query<(&mut Transform, &mut Velocity), (With<Minion>, Without<Player>)>,
    mut player_query: Query<&mut Transform, With<Player>>) {

    let player = player_query.single_mut();
    for mut minion in minion_query.iter_mut() {
        if minion.0.translation.distance(player.translation) <  150.0 { return; }
        
        let direction = (player.translation - minion.0.translation).normalize();
        minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
}
