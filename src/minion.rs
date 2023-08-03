use std::borrow::BorrowMut;
use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct Minion;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_minion)
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
    }, Minion));
}


pub fn minion_movement(
    mut minion_query: Query<&mut Transform, (With<Minion>, Without<Player>)>,
    mut player_query: Query<&mut Transform, With<Player>>) {

        let mut player = player_query.single_mut();

        let mut combinations = minion_query.iter_combinations_mut();
        while let Some([mut t1, t2]) = combinations.fetch_next(){
            if(t1.translation.distance(t2.translation) < 70.0){
                let next = (t2.translation - t1.translation);
                t1.borrow_mut().translation -= next.normalize();
                // let test = t1.translation.lerp(t2.translation, 0.8);
                // t1.borrow_mut().translation.x += test.x * time.delta_seconds();
                // t1.borrow_mut().translation.y += test.y * time.delta_seconds();
            }
        }

        for mut minion in minion_query.iter_mut() {
            println!("{:?}", minion.translation.distance(player.translation));
            if(minion.translation.distance(player.translation) > 150.0)
            {
                let next = (player.translation - minion.translation);
                minion.translation += next.normalize();
            }
        }


}