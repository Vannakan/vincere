use bevy::prelude::*;
use crate::{builder::components::*, change_state, Velocity, Campsite};

pub fn deposit_campsite( mut commands: Commands,
    mut builder_query: Query<(Entity, &mut Transform, &mut Velocity, &mut Builder), With<DepositCampsite>>,
    mut campsite_query: Query<&mut Campsite, Without<Builder>>)
    {
        if campsite_query.is_empty() {return;}

        let mut campsite = campsite_query.single_mut();
        for mut builder in builder_query.iter_mut() 
        {
            if builder.3.trees <= 0 {
                change_state::<DepositCampsite>(&mut commands, builder.0, FindTree); continue;
            } 

            builder.3.trees -= 1;
            campsite.trees += 1;             
        }
    }

pub fn move_to_campsite(
    mut commands: Commands,
    mut builder_query: Query<(Entity, &mut Transform, &mut Velocity, &mut Builder), With<MoveToCampsite>>,
    mut campsite_query: Query<&mut Transform, (With<Campsite>, Without<Builder>)>) 
    {
    if builder_query.is_empty() { return; }
    
    let transform = campsite_query.single_mut();

    for mut builder in builder_query.iter_mut() 
    {
        if builder.1.translation.distance(transform.translation) <  75.0 
        { 
                change_state::<MoveToCampsite>(&mut commands, builder.0, DepositCampsite); continue; 
        }
        let direction = (transform.translation - builder.1.translation).normalize();
        builder.2.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
 }
