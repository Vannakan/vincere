use bevy::prelude::*;

use crate::{player::Player, Velocity, Minion, Campsite, Tree};

#[derive(Component)]
pub struct Builder{
    trees: i32,
    state: BuilderState
}

enum BuilderState {
    TREE,
    CAMPSITE,
    FIND
}

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App){
        app//.add_systems(Startup, spawn_builder)
     //   .add_systems(Startup, spawn_builder2)      
        .add_systems(Update, move_to_campsite)
         .add_systems(Update, builder_movement)
        .add_systems(Update, move_to_tree);
    }
}

#[derive(Event)]
pub struct FindTree(Vec3);

// pub fn find_nearest_tree(mut reader: EventReader<FindTree>, query: Query<&Transform, With<Tree>>){
//     for find in reader.iter(){
//         for tree in query.iter().(|t,y| {
//             t.translation.distance(find.0)
//         }){
    
//         }
//     }
// }

pub fn spawn_builder2(mut commands:  &mut Commands, asset_server: &Res<AssetServer>){
    let texture = asset_server.load("builder.png");
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3 { z: 1.0, x: 0.0, y: 400.0},
            ..default()
        },
        ..default()
    }, 
    Builder{
        trees: 0,
        state: BuilderState::TREE
    },
    Velocity(Vec3::default())));
}

pub fn spawn_builder(mut commands: &mut Commands, asset_server: &Res<AssetServer>){
    let texture = asset_server.load("builder.png");
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3 { z: 1.0, x: 0.0, y: 0.0},
            ..default()
        },
        ..default()
    }, 
    Builder{
        trees: 0,
        state: BuilderState::TREE
    },
    Velocity(Vec3::default())));
}

pub fn move_to_campsite(
    mut builder_query: Query<(&mut Transform, &mut Velocity, &mut Builder), (With<Builder>, Without<Player>, Without<Minion>, Without<Campsite>)>,
    mut campsite_query: Query<(&mut Transform, &mut Campsite)>) {

    if builder_query.is_empty() { return; }
    
    let (transform, mut campsite) = campsite_query.single_mut();
    for mut minion in builder_query.iter_mut().filter(|b| {
        if let BuilderState::CAMPSITE = b.2.state{
            return true;
        }else {
            return false;
        }
    }) {
            if(minion.2.trees <= 0) { minion.2.state = BuilderState::TREE; continue;}
            if minion.0.translation.distance(transform.translation) <  150.0 { if minion.2.trees > 0 { minion.2.trees -=1; campsite.trees +=1; println!("campsite trees: {}", campsite.trees);} continue ; }
            let direction = (transform.translation - minion.0.translation).normalize();
            minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
        }
 }

 pub fn move_to_tree(
    mut builder_query: Query<(&mut Transform, &mut Velocity, &mut Builder), (With<Builder>, Without<Player>, Without<Minion>, Without<Tree>)>,
    mut tree_query: Query<(&mut Transform), With<Tree>>) {

    let tree = tree_query.single_mut();
    for mut minion in builder_query.iter_mut() {
        if let BuilderState::TREE = minion.2.state {
            if(minion.2.trees >= 250) { minion.2.state = BuilderState::CAMPSITE; continue;}
            if minion.0.translation.distance(tree.translation) <  150.0 { minion.2.trees +=1;  println!("{} trees", minion.2.trees); continue; }

            let direction = (tree.translation - minion.0.translation).normalize();
            minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
        }
    }
   
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

// pub fn builder_avoidance(mut minion_query: Query<(&mut Transform, &mut Velocity),(With<Builder>, Without<Player>, Without<Minion>)>){
//     let mut combinations = minion_query.iter_combinations_mut();
//     while let Some([mut t1, t2]) = combinations.fetch_next(){
//         if t1.0.translation.distance(t2.0.translation) > 50.0 { return;}

//         let direction = (t2.0.translation - t1.0.translation).normalize();
//         t1.borrow_mut().1.0 -= Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}
//     }
// }

// pub fn builder_follow_player(
//     mut minion_query: Query<(&mut Transform, &mut Velocity), (With<Builder>, Without<Player>, Without<Minion>)>,
//     mut player_query: Query<&mut Transform, With<Player>>) {

//     let player = player_query.single_mut();
//     for mut minion in minion_query.iter_mut() {
//         if minion.0.translation.distance(player.translation) <  150.0 { return; }
        
//         let direction = (player.translation - minion.0.translation).normalize();
//         minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
//     }
// }
