use bevy::prelude::*;
use rand::Rng;

use crate::{Velocity, Campsite, Tree, change_state, get_nearest_entity, HasUi, BindUi};

#[derive(Component)]
pub struct Builder{
    trees: i32,
    last_action: f32,
}

#[derive(Component)]
pub struct Target(Entity);

#[derive(Component)]
pub struct FindTree;

#[derive(Component)]
pub struct MoveToTree {
    entity: Entity
}

#[derive(Component)]
pub struct CutTree;

#[derive(Component)]
pub struct MoveToCampsite;

#[derive(Component)]
pub struct DepositCampsite;

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, (
            find_tree,
            cut_tree, 
            move_to_tree,
            move_to_campsite,
            deposit_campsite,
    ));}
}

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
                source: asset_server.load(format!("audio/woodcut-{}.ogg", rng.gen_range(0..=4))),
                settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn,speed: 0.5, ..Default::default() }
                });
            }
        }
}

fn move_to_tree(mut commands: Commands, mut query: Query<( Entity, &MoveToTree, &Transform,&mut Velocity )>, tree_query: Query<(Entity, &Transform, &Tree)>){
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

pub fn spawn_builder(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3, writer: &mut EventWriter<BindUi>){
    let texture = asset_server.load("builder.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: position,
            ..default()
        },
        ..default()
    }, 
    Builder{
        trees: 0,
        last_action: 0.0
    },
    FindTree,
    HasUi,
    Velocity(Vec3::default()))).id();

    writer.send(BindUi(entity, "Builder".to_string()));
}
