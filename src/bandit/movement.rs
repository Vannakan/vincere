use bevy::prelude::*;

use crate::{Velocity, Minion, get_nearest_entity, change_state, FoundTarget, MoveTo, HasTarget, Attack};

use super::{components::{Bandit, Idle, MoveToMinion, AttackMinion}, attack::reset_bandit_state};


const ENEMY_RANGE: f32 = 500.0;


// pub fn bandit_found_target(mut commands: Commands, query: Query<(Entity, &Bandit, &FoundTarget<Minion>), Added<FoundTarget<Minion>>>)
// {
//     if query.is_empty() { return; }
//     for evt in query.iter(){
//         println!("Did!");
//         if let Ok(entity) = query.get(evt.0){
//             if let Some(mut e) = commands.get_entity(entity.0){
//                 e.insert(HasTarget{ target: Some(evt.2.to)});
//                 e.remove::<FoundTarget<Minion>>();
//                 change_state::<Idle>(&mut commands, entity.0, MoveTo)
//             }
//         }
//     }
// }

pub fn bandit_found_target(mut commands: Commands, query: Query<(Entity, &Bandit), Added<HasTarget>>){
    if query.is_empty() { return; }
    for evt in query.iter(){
        println!("Did!");
        if let Ok(entity) = query.get(evt.0){
            if let Some(mut e) = commands.get_entity(entity.0){
                // e.insert(HasTarget{ target: Some(evt.2.to)});
                // e.remove::<FoundTarget<Minion>>();
                change_state::<Idle>(&mut commands, entity.0, MoveTo)
            }
        }
    }
}

pub fn move_to_minion(
    mut commands: Commands,
    mut bandit_query: Query<(Entity, &mut Transform, &mut Velocity, &MoveTo, &HasTarget), With<Bandit>>,
    mut minion_query: Query<(Entity, &Transform), (With<Minion>, Without<Bandit>)>,)
{
    if bandit_query.is_empty(){ return;}
    let minions  = minion_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for mut bandit in bandit_query.iter_mut()
    {
        let target = match bandit.4.target 
        {
            Some(b) => b,
            _ => {
                reset_bandit_state(&mut commands, bandit.0);
                continue;
            }
        };

        match minions.iter().find(|t| t.0 == target)
        {
            Some(m) => {
                if bandit.1.translation.distance(m.1.translation) <  150.0 
                {   change_state::<MoveTo>(&mut commands, bandit.0, Attack); continue;
                 }
                let direction = (bandit.1.translation - m.1.translation).normalize();
                bandit.2.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0};
            },
            _ => {
                reset_bandit_state(&mut commands, bandit.0);
                continue;
            }
        };
    }
}

pub fn default_bandit(mut commands: Commands, query: Query<Entity, (With<Bandit>, Without<MoveTo>, Without<HasTarget>, Without<Attack>, Without<Idle>)>){
    if query.is_empty() { return; }
    for minion in query.iter(){
        reset_bandit_state(&mut commands, minion);
    }
}

