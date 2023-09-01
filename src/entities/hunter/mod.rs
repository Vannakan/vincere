use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{common::components::Velocity, rabbit::Rabbit, combat::components::{FindTarget, HasTarget}, common::components::Targetable, common::components::Target, get_nearest_entity, bandit::components::Idle};

#[derive(Component)]
pub struct Hunter;

#[derive(Component)]
pub struct EntityStats{
   range: f32
}

pub fn spawn_hunter(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let texture = asset_server.load("hunter.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
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
    Hunter,
    FindTarget::<Rabbit> {
        phantom: PhantomData
    },
    Velocity(Vec3::default()))).id();
}

pub fn hunter_shoot(query: Query<(Entity, &Transform), With<Hunter>>){
    if query.is_empty() { return; }
    for hunter in query.iter() {

    }
}

// #[derive(Component)]
// pub struct StateTransition<TFrom, TTo: Component> {
//     pub phantom: PhantomData<TFrom>,
//     pub phantom_to: PhantomData<TTo>
// }

// #[derive(Component)]
// pub struct HasTarget2{
//     pub entity: Entity,
//     pub position: Vec3
// }


// #[derive(Component)]
// pub enum TargetType {
//     ENEMY,
// }

// pub fn update_target_translation(mut query: Query<&mut HasTarget2>, other: Query<(Entity, &Targetable, &Transform)>) {
//      for mut has in query.iter_mut() {
//         let target = other.get(has.entity).unwrap();
//         has.position = target.2.translation;
//      }
// }

// pub fn global_find_target_with2<TWith: Component, Type: Component>(
//     mut commands: Commands,
//     query: Query<(Entity, &mut Transform), (With<FindTarget<TWith>>, Without<TWith>, Without<HasTarget2>)>,
//     mut target_query: Query<(Entity, &Transform), (With<TWith>)>,
//     )
// {
//     if target_query.is_empty() || query.is_empty()  { return; }
//     let mut targets  = target_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
//     for entity in query.iter(){
//         let nearest = get_nearest_entity(&mut targets, entity.1.translation);
//         println!("FOUND TARGET");
//         let mut e = commands.entity(entity.0);
//         e.insert(HasTarget2{ entity: nearest.0, position: nearest.1.translation});
//         e.insert(StateTransition::<Idle, HasTarget2>{ phantom_to: PhantomData, phantom: PhantomData});
//         e.remove::<FindTarget<HasTarget2>>();
//     }           
// } 

// pub fn handle_hunter_found_target_transition(mut commands: Commands, mut query: Query<Entity, &StateTransition<Idle, HasTarget2>>){
//     if query.is_empty() { return; }

//     for hunter in query.iter_mut() {
//         let mut entity = commands.get_entity(hunter).unwrap();
//         entity.insert(MoveTo);
//         entity.remove::<StateTransition<Idle, HasTarget2>>();
//     }
// }

// pub fn handle_move_to_target(mut commands: Commands, mut move_query: Query<(Entity, &HasTarget2, &MoveTo, &mut Velocity, &Transform)>){
//     for (entity, target, moveTo, mut velocity, transform) in move_query.iter_mut() {
        
//         if transform.translation.distance(target.position) > 300.0 // range should be defined on an entity level
//         {
//             let direction = (target.position - transform.translation).normalize();
//             velocity.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0};
//         } else {
//             let mut test = commands.get_entity(entity).unwrap();
//             test.remove::<MoveTo>();
//             test.insert(StateTransition::<MoveTo, Idle>{ phantom_to: PhantomData, phantom: PhantomData});
//         }  
//     }
// }

// pub fn hunter_moved_to_target(mut query: Query<(Entity, &StateTransition<MoveTo, Idle>, &HasTarget2)>){
//     // if target is close
//     // if target.type is enemy
//     // switch to attack state
// }
