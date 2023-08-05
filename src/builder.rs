use bevy::prelude::*;

use crate::{player::Player, Velocity, Minion, Campsite, Tree, change_state, get_nearest_entity};

#[derive(Component)]
pub struct Builder{
    trees: i32,
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
        app .add_systems(Update, (
            find_tree,
            cut_tree, 
            move_to_tree,
            move_to_campsite,
            deposit_campsite,
            builder_movement
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
            println!("CAMPSITE: {}, BUILDER:{}", campsite.trees, builder.3.trees );              
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
        if builder.1.translation.distance(transform.translation) <  150.0 
        { 
                change_state::<MoveToCampsite>(&mut commands, builder.0, DepositCampsite); continue; 
        }
        let direction = (transform.translation - builder.1.translation).normalize();
        builder.2.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
 }

pub fn cut_tree(mut commands: Commands, mut query: Query<(Entity, &mut CutTree, &mut Builder)>){
    for mut builder in query.iter_mut(){
        if builder.2.trees >= 250 {
            change_state::<CutTree>(&mut commands, builder.0, MoveToCampsite)
        }else {
            builder.2.trees += 1;
        }
    }
}


fn move_to_tree(mut commands: Commands, mut query: Query<( Entity, &MoveToTree, &Transform,&mut Velocity )>, tree_query: Query<(Entity, &Transform, &Tree)>){
    if tree_query.is_empty() 
    { 
        return;
    }
    let trees = tree_query.iter().collect::<Vec<(Entity, &Transform, &Tree)>>();
    for mut builder in query.iter_mut ()
    {
        let index = trees.iter().position(|p| p.0 == builder.1.entity).unwrap();
        let tree_entity = trees[index];

        if tree_entity.1.translation.distance(builder.2.translation) <  150.0{
            change_state::<MoveToTree>(&mut commands, builder.0, CutTree);
            continue;
        }

        let direction = (builder.2.translation - tree_entity.1.translation).normalize();
        builder.3.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}   
    }
}

pub fn find_tree(mut commands: Commands, mut query: Query<(Entity, &Transform, &mut FindTree)>, mut tree_query: Query<(&Tree, Entity, &Transform)>){
    let mut trees = tree_query.iter_mut().map(|x| (x.1, x.2)).collect::<Vec<(Entity, &Transform)>>();
    for builder in query.iter_mut(){
        let nearest = get_nearest_entity(&mut trees, builder.1.translation);
        change_state::<FindTree>(&mut commands,builder.0, MoveToTree { entity: nearest.0 });
    }
}

pub fn spawn_builder(mut commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3){
    let texture = asset_server.load("builder.png");
    commands.spawn((SpriteBundle {
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
    },
    FindTree,
    Velocity(Vec3::default())));
}

pub fn builder_movement(
    mut builder_query: Query<(&mut Transform, &mut Velocity),(With<Builder>, Without<Player>, Without<Minion>)>, time: Res<Time>,){
    for minion in builder_query.iter_mut(){
        let (mut transform, mut velocity) = minion;

        transform.translation += velocity.0 * time.delta_seconds();
    
        if velocity.0.x >= -0.1 && velocity.0.x <= 0.1 && velocity.0.y <= 0.1 && velocity.0.y >= -0.1
        {
            velocity.0 = Vec3::default();
        }
        else {
            velocity.0 = velocity.0.lerp(Vec3::default(), 0.1)
        }
    }
}
