use std::marker::PhantomData;

use bevy::prelude::*;

use crate::combat::components::{Attacks, Damage, HasTarget, FindTarget};
use crate::combat::events::AttackEvent;
use crate::common::components::Attack;
use crate::common::components::MoveTo;
use crate::constants::BANDIT_ATTACK_RANGE;
use crate::entities::minion::Minion;
use crate::change_state;

use super::components::{Bandit, Idle};

pub fn attack_minion(
    mut commands: Commands,
    mut bandit_query: Query<(Entity, &mut Attacks, &Damage, &Attack, &Transform, &HasTarget), With<Bandit>>,
    mut minion_query: Query<(Entity, &Transform), (With<Minion>, Without<Bandit>)>,
    mut writer: EventWriter<AttackEvent>,
    time: Res<Time>)
{  
    if bandit_query.is_empty() || minion_query.is_empty() { return; }
    let minions  = minion_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for mut bandit in bandit_query.iter_mut()
    {
        let target = match bandit.5.target {
            Some(t) => t,
            _ => {
                {
                    reset_bandit_state(&mut commands, bandit.0);
                    continue;
                }
            }
        };

        match minions.iter().find(|t| t.0 == target)
        {
            Some(t) => 
            {
                if bandit.4.translation.distance(t.1.translation) > BANDIT_ATTACK_RANGE 
                    { 
                        change_state::<Attack>(&mut commands, bandit.0, MoveTo) 
                    }
                if bandit.1.last_attacked > time.elapsed_seconds() - 1.5   { continue; }
                writer.send(AttackEvent { from:bandit.4.clone() , to: target , damage: bandit.2.0 });   
                bandit.1.last_attacked = time.elapsed_seconds();
            },
            _ => {
                reset_bandit_state(&mut commands, bandit.0);
                continue;
            }
        };
    }
}

pub fn reset_bandit_state(mut commands: &mut Commands, entity: Entity) {

    let mut e = commands.get_entity(entity).unwrap();
    e.remove::<Attack>();
    e.remove::<HasTarget>();
    e.remove::<MoveTo>();
    e.insert(FindTarget::<Minion> {
        phantom: PhantomData
    });
    e.insert(Idle);
}

pub fn enemy_defeated(mut commands: Commands, query: Query<(Entity, &HasTarget), With<Bandit>>){
    for entity in query.iter(){
        let target = match(entity.1.target){
            Some(t) => t,
            _ => {
                reset_bandit_state(&mut commands, entity.0);
                continue;
            }
        };

        match commands.get_entity(target) {
            Some(_) => continue,
            _ => {
                reset_bandit_state(&mut commands, entity.0);
                continue;
            }
        };
    }
}