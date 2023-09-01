use std::marker::PhantomData;

use bevy::prelude::*;
use crate::combat::components::{FindTarget, HasTarget};
use crate::common::constants::ACTION_COOLDOWN;
use crate::entities::minion::MoveTo;
use crate::game::campsite::Campsite;
use crate::game::tree::Tree;
use crate::{builder::components::*, change_state};
use crate::common::components::Velocity;

pub fn find_campsite(mut commands: Commands, mut query: Query<(Entity, &Transform), (With<Builder>, With<FindTarget::<Campsite>>)>, mut campfire_query: Query<(&Campsite, Entity, &Transform)>)
{
    if campfire_query.is_empty() { return; }
    let campsite = campfire_query.single();
    for builder in query.iter_mut()
    {
        change_state::<FindTree>(&mut commands,builder.0, MoveTo );
        
        commands.get_entity(builder.0).unwrap().remove::<FindTarget::<Campsite>>();
        commands.get_entity(builder.0).unwrap().insert(MovingTo::<Campsite>{ phantom: PhantomData});
        commands.get_entity(builder.0).unwrap().insert(HasTarget{ target: Some(campsite.1)});
    }
}

pub fn move_to_campsite(
    mut commands: Commands,
    mut query: Query<( Entity, &MoveTo, &Transform,&mut Velocity ), (With<MovingTo::<Campsite>>)>, 
    campsite_query: Query<(Entity, &Transform), With<Campsite>>)
{
    if campsite_query.is_empty() || query.is_empty() {return;}
    let campsite = campsite_query.single();
    for mut builder in query.iter_mut ()
    {
        if campsite.1.translation.distance(builder.2.translation) <  75.0
        {
            // get entity and use once
            commands.get_entity(builder.0).unwrap().remove::<MovingTo::<Campsite>>();
            change_state::<MoveTo>(&mut commands, builder.0, DepositCampsite); continue; 
        }

        let direction = (builder.2.translation - campsite.1.translation).normalize();
        builder.3.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}   
    }
}

pub fn deposit_campsite( mut commands: Commands,
    mut builder_query: Query<(Entity, &mut Builder, &mut Action), With<DepositCampsite>>,
    mut campsite_query: Query<&mut Campsite, Without<Builder>>,
    time: Res<Time>)
    {
        if campsite_query.is_empty() {return;}

        let mut campsite = campsite_query.single_mut();
        for mut builder in builder_query.iter_mut() 
        {
            if builder.2.last_action > time.elapsed_seconds() - ACTION_COOLDOWN { continue; }

            if builder.1.trees <= 0 {
                commands.get_entity(builder.0).unwrap().remove::<HasTarget>();
               change_state::<DepositCampsite>(&mut commands, builder.0,FindTarget::<Tree>{
                phantom:PhantomData
            }); continue;
            } 

            builder.1.trees -= 1;
            campsite.trees += 1;             
            builder.2.last_action = time.elapsed_seconds();
        }
    }

