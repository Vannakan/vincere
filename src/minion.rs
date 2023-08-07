use std::borrow::BorrowMut;
use bevy::prelude::*;

use crate::{ player::Player, Velocity, Bandit, change_state, get_nearest_entity, avoidance, BindUi, HasUi, Attack, Health, Damage, Attacks };

#[derive(Component)]
pub struct Minion;

#[derive(Component)]
pub struct FollowPlayer;


#[derive(Component)]
pub struct MoveToEnemy(Entity, Transform);

#[derive(Component)]
pub struct AttackEnemy(Entity, Transform); //Trasnform of entity attacking

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App){
        app      
         .add_systems(Update, minion_follow_player)
         .add_systems(Update, minion_avoidance)
         .add_systems(Update, check_enemy_range)
         .add_systems(Update, move_to_enemy)
         .add_systems(Update, attack_enemy)
         .add_systems(Update, on_spawn_minions)
         .add_event::<SpawnMinion>();
    }
} 

#[derive(Event)]
pub struct SpawnMinion(pub Vec3);

fn on_spawn_minions(mut commands: Commands, asset_server: Res<AssetServer>, mut writer: EventWriter<BindUi>, mut reader:EventReader<SpawnMinion>){
    for evt in  reader.iter(){
        println!("SPAWN");
        let texture = asset_server.load("knight.png");
        let entity = commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            transform: Transform{
                translation: evt.0,
                ..default()
            },
            ..default()
        }, 
        Health{
            starting: 30.0,
            current: 30.0
        },
        Damage(5.0),
        Minion,
        FollowPlayer,
        HasUi,
        Attacks {
             last_attacked: 0.0
        },
        Velocity(Vec3::default()))).id();
    
    
        writer.send(BindUi(entity, "Knight".to_string()));
    }
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
    mut minion_query: Query<(Entity, &mut Transform, &mut Velocity, &MoveToEnemy), With<Minion>>,
    mut bandit_query: Query<(Entity, &Transform), (With<Bandit>, Without<Minion>)>,)
    {
        let bandits  = bandit_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
        for mut minion in minion_query.iter_mut(){

            if let Some(bandit) = bandits.iter().find(|t| {
                t.0 == minion.3.0
            }){
                if minion.1.translation.distance(bandit.1.translation) <  150.0
                { 
                    change_state::<MoveToEnemy>(&mut commands, minion.0, AttackEnemy(bandit.0, *minion.1)); 
                    continue;
                }

                let direction = (bandit.1.translation - minion.1.translation).normalize();
                minion.2.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0} ;
            }else {
                change_state::<FollowPlayer>(&mut commands, minion.0, FollowPlayer);
            }
        }
}


pub fn attack_enemy(
    mut commands: Commands,
    mut minion_query: Query<(Entity, &mut Attacks, &Damage, &AttackEnemy, &mut Transform), With<Minion>>,
    mut bandit_query: Query<(Entity, &Transform), (With<Bandit>, Without<Minion>)>,
    mut writer: EventWriter<Attack>,
    time: Res<Time>)
    {  
        if minion_query.is_empty() { return; }
        let bandits  = bandit_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
       
        for mut minion in minion_query.iter_mut(){
                if let Some(bandit) = bandits.iter().find(|t| t.0 == minion.3.0)
                {
                    if minion.4.translation.distance(bandit.1.translation) > 150.0 
                    { 
                        change_state::<AttackEnemy>(&mut commands, minion.0, MoveToEnemy(minion.3.0, *bandit.1));
                        continue;
                    }

                    if minion.1.last_attacked > time.elapsed_seconds() - 1.5   { continue; }
                    writer.send(Attack { from:*minion.4 , to: minion.3.0 , damage: minion.2.0 });   
                    minion.1.last_attacked = time.elapsed_seconds();
                } else {
                    change_state::<AttackEnemy>(&mut commands, minion.0, FollowPlayer)
                }
        }
}

const ENEMY_RANGE: f32 = 500.0;

pub fn check_enemy_range(
    mut commands: Commands,
    mut minion_query: Query<(Entity, &mut Transform, &mut Velocity), (With<Minion>, With<FollowPlayer>)>,
    mut bandit_query: Query<(Entity, &Transform), (With<Bandit>, Without<Minion>)>)
    {
        if bandit_query.is_empty(){return;}

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
    mut player_query: Query<&mut Transform, With<Player>>) 
    {

    let player = player_query.single_mut();
    for mut minion in minion_query.iter_mut() {
        if minion.0.translation.distance(player.translation) <  150.0 { continue; }
        
        let direction = (player.translation - minion.0.translation).normalize();
        minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
}
