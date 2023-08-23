use std::marker::PhantomData;

use bevy::prelude::*;
use rand::Rng;
use crate::combat::components::{HasTarget, FindTarget};
use crate::common::constants::ACTION_COOLDOWN;
use crate::{change_state,Tree, MoveTo, Campsite};
use crate::common::components::Velocity;

use super::components::{CutTree, Builder, Action, MovingTo};


const MAX_TREES: i32 = 10;

                              
pub fn cut_tree(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CutTree, &mut Builder, &mut Action)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>)
{
    let mut rng = rand::thread_rng();
    for mut builder in query.iter_mut()
    {
        if builder.3.last_action > time.elapsed_seconds() - ACTION_COOLDOWN { continue; }
        if builder.2.trees >= MAX_TREES {
            change_state::<CutTree>(&mut commands, builder.0,  FindTarget::<Campsite>{phantom: PhantomData})
           
        }
        else 
        {
            builder.2.trees += 1;
            builder.3.last_action = time.elapsed_seconds();
            commands.spawn(AudioBundle{
                source: asset_server.load(format!("audio/woodcut-{}.ogg", rng.gen_range(0..=3))),
                settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn,speed: 0.5, ..Default::default() }
                });
            }
        }
}

pub fn move_to_tree(mut commands: Commands, mut query: Query<( Entity, &MoveTo, &Transform,&mut Velocity, &HasTarget ), (Without<MovingTo::<Campsite>>)>, tree_query: Query<(Entity, &Transform, &Tree)>){
    if tree_query.is_empty() || query.is_empty() {return;}
    for mut builder in query.iter_mut ()
    {

        let target = match builder.4.target {
            Some(e) => e,
            _ => {continue;} //remove state
        };

        let tree = match tree_query.get(target){
            Ok(t) => t,
            _ => {continue;}
        };

        if tree.1.translation.distance(builder.2.translation) <  75.0
        {
            change_state::<MoveTo>(&mut commands, builder.0, CutTree);
            continue;
        }

        let direction = (builder.2.translation - tree.1.translation).normalize();
        builder.3.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}   
    }
}


pub fn builder_found_target(mut commands: Commands, query: Query<(Entity, &Builder), Added<HasTarget>>){
    if query.is_empty() { return; }
    for evt in query.iter(){
        if let Ok(entity) = query.get(evt.0){
            if let Some(mut e) = commands.get_entity(entity.0){
                println!("Changed state from FindTree to MoveTo!");
                change_state::<FindTarget::<Tree>>(&mut commands, entity.0, MoveTo);
            }
        }
    }
}
