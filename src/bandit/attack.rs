use bevy::prelude::*;

use crate::{Attacks, Damage, Attack, Minion, change_state};

use super::components::{AttackMinion, Bandit, MoveToMinion, Idle};

pub fn attack_minion(
    mut commands: Commands,
    mut minion_query: Query<(Entity, &mut Attacks, &Damage, &AttackMinion, &Transform), With<Bandit>>,
    mut bandit_query: Query<(Entity, &Transform), (With<Minion>, Without<Bandit>)>,
    mut writer: EventWriter<Attack>,
    time: Res<Time>)
{  
    if minion_query.is_empty() { return; }
    let bandits  = bandit_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for mut minion in minion_query.iter_mut()
    {
        if let Some(bandit) = bandits.iter().find(|t| t.0 == minion.3.0)
        {
            let test = *bandit.1;
            if minion.4.translation.distance(bandit.1.translation) > 75.0 
                { 
                    change_state::<AttackMinion>(&mut commands, minion.0, MoveToMinion(minion.3.0,test)) 
                }
            if minion.1.last_attacked > time.elapsed_seconds() - 1.5   { continue; }
        
            writer.send(Attack { from:*minion.4 , to: minion.3.0 , damage: minion.2.0 });   
            minion.1.last_attacked = time.elapsed_seconds();
        } else 
        {
            change_state::<AttackMinion>(&mut commands, minion.0, Idle)
        }
    }
}
