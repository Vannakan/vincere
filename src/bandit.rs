use std::borrow::BorrowMut;
use bevy::{prelude::*, window::PrimaryWindow};
use crate::{ Velocity, Minion, change_state, get_nearest_entity, avoidance, move_to, Damage, Health};

#[derive(Component)]
pub struct Bandit;

#[derive(Component)]
pub struct Idle;


#[derive(Component)]
pub struct MoveToMinion(Entity, Transform);

#[derive(Component)]
pub struct AttackMinion(Entity);

pub struct BanditPlugin;

impl Plugin for BanditPlugin {
    fn build(&self, app: &mut App){
        app     
         .add_systems(Update, add_bandit)
         .add_systems(Update, bandit_avoidance)
         .add_systems(Update, bandit_movement)
         .add_systems(Update, check_minion_range)
         .add_systems(Update, move_to_minion);
    }
}

// DUPLICATE OF MINION & BUILDER

const ENEMY_RANGE: f32 = 500.0;

fn check_minion_range(
    mut commands: Commands,
    bandit_query: Query<(Entity, &mut Transform, &mut Velocity), (With<Bandit>, With<Idle>, Without<Minion>)>,
    mut minion_query: Query<(Entity, &Transform), (With<Minion>, Without<Bandit>)>)
    {
        if bandit_query.is_empty() || minion_query.is_empty(){
            return;
        }

        let mut minions  = minion_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
        for bandit in bandit_query.iter(){
            let minion = get_nearest_entity(&mut minions, bandit.1.translation);
            if minion.1.translation.distance(bandit.1.translation) > ENEMY_RANGE {
                continue;
            }
     
            change_state::<Idle>(&mut commands, bandit.0, MoveToMinion(minion.0, minion.1))
        }
}

fn move_to_minion(
    mut commands: Commands,
    mut bandit_query: Query<(Entity, &mut Transform, &mut Velocity, &MoveToMinion), With<Bandit>>)
    {
        if bandit_query.is_empty(){ return;}
        for mut bandit in bandit_query.iter_mut()
        {
            let minion = bandit.3;
            if bandit.1.translation.distance(minion.1.translation) <  150.0 {   change_state::<MoveToMinion>(&mut commands, bandit.0, AttackMinion(minion.0)); continue; }
            let direction = (bandit.1.translation - minion.1.translation).normalize();
            bandit.2.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0};
        }
}

/////////////////////

fn add_bandit(mut commands: Commands, mut asset_server: ResMut<AssetServer>, input: Res<Input<KeyCode>>, q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>){
        if let Some(position) = q_windows.single().cursor_position(){
        let (camera, camera_transform) = camera_q.single();
        if(input.just_released(KeyCode::B)){
            spawn_bandit(&mut commands, &mut asset_server, Vec3::from((camera.viewport_to_world_2d(camera_transform, position).unwrap(), 1.0)))
        }
    }
}

fn spawn_bandit(mut commands: &mut Commands, asset_server: &mut ResMut<AssetServer>, position: Vec3){
    let texture = asset_server.load("bandit.png");
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: position,
            ..default()
        },
        ..default()
    }, 
    Idle,
    Bandit,
    Health(10.0),
    Damage(1.0),
    Velocity(Vec3::default())));
}


pub fn bandit_movement(
    mut bandit_query: Query<(&mut Transform, &mut Velocity),With<Bandit>>, time: Res<Time>,){
    for bandit in bandit_query.iter_mut(){
        let (mut transform, mut velocity) = bandit;

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

pub fn bandit_avoidance(mut bandit_query: Query<(&mut Transform, &mut Velocity), With<Bandit>>){
    let mut combinations = bandit_query.iter_combinations_mut();
    while let Some([mut t1, t2]) = combinations.fetch_next(){
        if t1.0.translation.distance(t2.0.translation) > 50.0 { continue;}
        let t1_pos = t1.0.translation;
        let t2_pos = t2.0.translation;
        t1.borrow_mut().1.0 += avoidance(t1_pos,t2_pos)
    }
}
