use bevy::prelude::*;

use crate::{Velocity, Minion, get_nearest_entity, change_state};

use super::components::{Bandit, Idle, MoveToMinion, AttackMinion};


const ENEMY_RANGE: f32 = 500.0;

pub fn check_minion_range(
    mut commands: Commands,
    bandit_query: Query<(Entity, &mut Transform, &mut Velocity), (With<Bandit>, With<Idle>, Without<Minion>)>,
    mut minion_query: Query<(Entity, &Transform), (With<Minion>, Without<Bandit>)>)
{
    if bandit_query.is_empty() || minion_query.is_empty(){return;}

    let mut minions  = minion_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for bandit in bandit_query.iter(){
        let minion = get_nearest_entity(&mut minions, bandit.1.translation);
        if minion.1.translation.distance(bandit.1.translation) > ENEMY_RANGE {
            continue;
        }
    
        change_state::<Idle>(&mut commands, bandit.0, MoveToMinion(minion.0, minion.1))
    }
}

pub fn move_to_minion(
    mut commands: Commands,
    mut bandit_query: Query<(Entity, &mut Transform, &mut Velocity, &MoveToMinion), With<Bandit>>,
    mut minion_query: Query<(Entity, &Transform), (With<Minion>, Without<Bandit>)>,)
{
    if bandit_query.is_empty(){ return;}
    let minions  = minion_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for mut bandit in bandit_query.iter_mut()
    {
        if let Some(minion) = minions.iter().find(|t| t.0 == bandit.3.0)
        {
            if bandit.1.translation.distance(minion.1.translation) <  75.0 
            {   
                change_state::<MoveToMinion>(&mut commands, bandit.0, AttackMinion(minion.0)); 
                continue;
            }
            let direction = (bandit.1.translation - minion.1.translation).normalize();
            bandit.2.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0};
        }
    }
}
