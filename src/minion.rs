use std::borrow::BorrowMut;
use bevy::prelude::*;

use crate::{player::Player, Velocity, Bandit, change_state, get_nearest_entity, avoidance, move_to };

#[derive(Event)]
pub struct Attack(pub Entity);


#[derive(Component)]
pub struct Attacks {
    last_attacked: f32
}

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct Minion;

#[derive(Component)]
pub struct FollowPlayer;


#[derive(Component)]
pub struct MoveToEnemy(Entity, Transform);

#[derive(Component)]
pub struct AttackEnemy(Entity);


pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_minion)      
         .add_systems(Update, minion_follow_player)
         .add_systems(Update, minion_avoidance)
         .add_systems(Update, minion_movement)
         .add_systems(Update, check_enemy_range)
         .add_systems(Update, move_to_enemy)
         .add_event::<Attack>()
         .add_systems(Update, (attack_system, attack_enemy))
         .add_systems(Update, kill);
    }
}

fn spawn_minion(mut commands: Commands, asset_server: Res<AssetServer>){
    let texture = asset_server.load("knight.png");
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
    Health(30.0),
    Damage(2.0),
    Minion,
    FollowPlayer,
    Attacks {
         last_attacked: 0.0
    },
    Velocity(Vec3::default())));

    let texture = asset_server.load("knight.png");
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
    Health(30.0),
    Damage(2.0),
    FollowPlayer,
    Attacks {
        last_attacked: 0.0
   },
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
        if t1.0.translation.distance(t2.0.translation) > 50.0 { continue;}
        let t1_position = t1.0.translation;
        t1.borrow_mut().1.0 -= avoidance(t2.0.translation, t1_position);
    }
}

pub fn move_to_enemy(
    mut commands: Commands,
    mut minion_query: Query<(Entity, &mut Transform, &mut Velocity, &MoveToEnemy), With<Minion>>)
    {
        for mut minion in minion_query.iter_mut(){

            let bandit = minion.3;
            if minion.1.translation.distance(bandit.1.translation) <  150.0 
            { 
                change_state::<MoveToEnemy>(&mut commands, minion.0, AttackEnemy(bandit.0)); 
                continue;
             }

            let direction = (bandit.1.translation - minion.1.translation).normalize();
            minion.2.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0} ;
        }
}

pub fn attack_system(mut events: EventReader<Attack>, mut query: Query<(Entity, &mut Health)>){
    for evt in events.iter(){
        let e = query.iter_mut().find(|(x, y)| {
            x == &evt.0
        });

        if let Some(mut e) = e {
            e.1.0 -= 5.0;
            println!("{:?}", e.1.0 )
        }
    }
}

pub fn kill(mut commands: Commands, query: Query<(Entity, &Health)>){
    for entity in query.iter(){
        if entity.1.0 <= 0.0 
        {
            commands.entity(entity.0).despawn();
        }
    }
}

pub fn attack_enemy(
    mut commands: Commands,
    mut minion_query: Query<(Entity, &mut Attacks, &Damage, &AttackEnemy), With<Minion>>,
    mut writer: EventWriter<Attack>,
    time: Res<Time>)
    {  
        if minion_query.is_empty() { return; }
       
        for mut minion in minion_query.iter_mut(){
                if let Some(entity) = commands.get_entity(minion.3.0){
                if minion.1.last_attacked > time.elapsed_seconds() - 3.0   { continue; }
                writer.send(Attack(minion.3.0));          
                minion.1.last_attacked = time.elapsed_seconds();
            } else {
                change_state::<AttackEnemy>(&mut commands, minion.0, FollowPlayer)
            }
        }
}


const ENEMY_RANGE: f32 = 350.0;

pub fn check_enemy_range(
    mut commands: Commands,
    mut minion_query: Query<(Entity, &mut Transform, &mut Velocity), (With<Minion>, With<FollowPlayer>)>,
    mut bandit_query: Query<(Entity, &Transform), (With<Bandit>, Without<Minion>)>)
    {
        if bandit_query.is_empty(){
            return;
        }

        let mut bandits  = bandit_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
        for minion in minion_query.iter_mut(){
            let bandit = get_nearest_entity(&mut bandits, minion.1.translation);
           
            if bandit.1.translation.distance(minion.1.translation) > ENEMY_RANGE {
                continue;
            }

            change_state::<FollowPlayer>(&mut commands, minion.0, MoveToEnemy(bandit.0, bandit.1))
        }
}

pub fn minion_follow_player(
    mut minion_query: Query<(&mut Transform, &mut Velocity), (With<Minion>, Without<Player>, With<FollowPlayer>)>,
    mut player_query: Query<&mut Transform, With<Player>>) {

    let player = player_query.single_mut();
    for mut minion in minion_query.iter_mut() {
        if minion.0.translation.distance(player.translation) <  150.0 { continue; }
        
        let direction = (player.translation - minion.0.translation).normalize();
        minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
}
