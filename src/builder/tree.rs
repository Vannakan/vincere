use bevy::prelude::*;
use rand::Rng;
use crate::{builder::components::*, change_state, Velocity, Tree, get_nearest_entity};


pub fn cut_tree(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CutTree, &mut Builder)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>)
{
    let mut rng = rand::thread_rng();
    for mut builder in query.iter_mut()
    {
        if builder.2.last_action > time.elapsed_seconds() - 3.0 { continue; }
        if builder.2.trees >= 3 {
            change_state::<CutTree>(&mut commands, builder.0, MoveToCampsite)
        }
        else 
        {
            builder.2.trees += 1;
            builder.2.last_action = time.elapsed_seconds();
            commands.spawn(AudioBundle{
                source: asset_server.load(format!("audio/woodcut-{}.ogg", rng.gen_range(0..=3))),
                settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn,speed: 0.5, ..Default::default() }
                });
            }
        }
}

pub fn move_to_tree(mut commands: Commands, mut query: Query<( Entity, &MoveToTree, &Transform,&mut Velocity )>, tree_query: Query<(Entity, &Transform, &Tree)>){
    if tree_query.is_empty() {return;}

    let trees = tree_query.iter().collect::<Vec<(Entity, &Transform, &Tree)>>();

    for mut builder in query.iter_mut ()
    {
        let index = trees.iter().position(|p| p.0 == builder.1.entity).unwrap();
        let tree_entity = trees[index];

        if tree_entity.1.translation.distance(builder.2.translation) <  75.0
        {
            change_state::<MoveToTree>(&mut commands, builder.0, CutTree);
            continue;
        }

        let direction = (builder.2.translation - tree_entity.1.translation).normalize();
        builder.3.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}   
    }
}

pub fn find_tree(mut commands: Commands, mut query: Query<(Entity, &Transform, &mut FindTree)>, mut tree_query: Query<(&Tree, Entity, &Transform)>)
{
    if tree_query.is_empty() { return; }
    let mut trees = tree_query.iter_mut().map(|x| (x.1, x.2)).collect::<Vec<(Entity, &Transform)>>();
    for builder in query.iter_mut()
    {
        let nearest = get_nearest_entity(&mut trees, builder.1.translation);
        change_state::<FindTree>(&mut commands,builder.0, MoveToTree { entity: nearest.0 });
    }
}
