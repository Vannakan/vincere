use bevy::prelude::{Bundle, Commands, Entity, Transform, Vec3};
use rand::Rng;

use std::fmt::Debug;

use crate::Velocity;

pub fn change_state<ToRemove: Bundle>(commands: &mut Commands, entity: Entity, add: impl Bundle){
    // CAN FIX THE ISSUE BY HAVING A SYSTEM THAT RUNS LAST THAT RESPAWNS ENTITIES MARKED TO BE DESPAWNED
    if let Some(mut entity) =  commands.get_entity(entity) {
        entity.remove::<ToRemove>();
        entity.insert(add);
    }
}

pub fn get_nearest_entity(candidates: &mut Vec<(Entity, &Transform)>, builder: Vec3) -> (Entity, Transform) {

    let fuck_off = candidates.clone();
    candidates.sort_by(|a,b| {
        match a.1.translation.distance(builder).partial_cmp(&b.1.translation.distance(builder)){
            Some(res) => res,
            None => {panic!("{:?}", fuck_off); }
        }
    });

    let (entity, transform) =  candidates.first().unwrap();
    return (*entity, **transform);
}


pub fn avoidance(a: Vec3, b: Vec3) -> Vec3{
    // Overlapping - Addresses issue where direction is NaN whenever the vec3s are the same
    if a == b {
        let mut rand = rand::thread_rng();
        return Vec3{x: rand.gen_range(-1.0..=1.0) * 25.0, y: rand.gen_range(-1.0..=1.0) * 25.0, z: 0.0}
    }
    let direction = (a - b).normalize();
    Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}
}

pub fn move_to(mut velocity: Vec3, current: Transform, target: Transform){
    let direction = (target.translation - current.translation).normalize();
    velocity += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0} ;
}


impl Debug for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Velocity").field(&self.0).finish()
    }
}