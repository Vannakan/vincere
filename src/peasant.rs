use std::f32::consts::E;

// peasants just wander around their camp
// can be recruited for gold
// follow you to the base
// can be attacked by bandits, cant fight back
// once at campsite, wander around campsite until they have been assigned a role (warrior, gatherer)
use bevy::prelude::*;

use crate::builder::spawn_builder;
use crate::combat::components::{Health, Damage};
use crate::common::components::FollowPlayer;
use crate::player::components::Player;
use crate::ui::components::HasUi;
use crate::ui::events::{BindUi, DestroyUi};
use crate::Campsite;
use crate::common::components::Velocity;
use crate::common::components::BoundingBox;


#[derive(Component)]
pub struct Peasant;

pub struct PeasantPlugin;

impl Plugin for PeasantPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (peasant_player_collision, peasant_follow_player, peasant_campfire_collision, peasant_ready_for_job));
    }
}

pub fn spawn_peasant(mut commands: Commands, asset_server: Res<AssetServer>, mut writer: EventWriter<BindUi>,){
        let texture = asset_server.load("peasant.png");
        let entity = commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            texture,
            transform: Transform{
                translation: Vec3::from([-100.0, -100.0, 1.0]),
                ..default()
            },
            ..default()
        }, 
        HasUi, //when bind ui, as the hasui component instead offr
        Health{
            starting: 30.0,
            current: 30.0
        },
        Peasant,
        Damage(5.0),
        BoundingBox {
            width: 32.0,
            height: 32.0
        },
        Velocity(Vec3::default()))).id();

        writer.send(BindUi(entity, "[E]".to_string()));
}


pub fn spawn_peasant2(mut commands: &mut Commands, asset_server: &Res<AssetServer>, mut writer: &mut EventWriter<BindUi>, pos: Vec2){
    let texture = asset_server.load("peasant.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: Vec3::from([pos.x, pos.y, 1.0]),
            ..default()
        },
        ..default()
    }, 
    HasUi, //when bind ui, as the hasui component instead offr
    Health{
        starting: 30.0,
        current: 30.0
    },
    Peasant,
    Damage(5.0),
    BoundingBox {
        width: 100.0,
        height: 100.0
    },
    Velocity(Vec3::default()))).id();

    writer.send(BindUi(entity, "[E]".to_string()));
}


pub fn peasant_player_collision(input: Res<Input<KeyCode>>, mut commands: Commands, mut player_q: Query<(Entity, &Player, &BoundingBox, &Transform)>, peasant_q: Query<(Entity, &Peasant, &BoundingBox, &Transform)>, asset_server: Res<AssetServer>){

    if(player_q.is_empty() || peasant_q.is_empty()) { return; }

    if input.just_released(KeyCode::E) == false { return ;}
    let (_,_, b_aabb, b_trans) = player_q.single_mut();

    for (a_entity, _, a_aabb, a_trans) in peasant_q.iter(){

        let a_pos = a_trans.translation;
        let b_pos = b_trans.translation;

        if (a_pos.x < b_pos.x + b_aabb.width && a_pos.x + a_aabb.width > b_pos.x && a_pos.y < b_pos.y + b_aabb.height && a_pos.y + a_aabb.height > b_pos.y)
            {
                println!("Following");
                commands.get_entity(a_entity).unwrap().insert(FollowPlayer);
            }
    }

}


pub fn peasant_campfire_collision(mut commands: Commands, mut campfire_q: Query<(Entity, &Campsite, &BoundingBox, &Transform)>, peasant_q: Query<(Entity, &Peasant, &BoundingBox, &Transform), With<FollowPlayer>>, asset_server: Res<AssetServer>){

    if(campfire_q.is_empty() || peasant_q.is_empty()) { return; }
    let (_,_, b_aabb, b_trans) = campfire_q.single_mut();
    for (a_entity, _, a_aabb, a_trans) in peasant_q.iter(){

        let a_pos = a_trans.translation;
        let b_pos = b_trans.translation;

        if (a_pos.x < b_pos.x + b_aabb.width && a_pos.x + a_aabb.width > b_pos.x && a_pos.y < b_pos.y + b_aabb.height && a_pos.y + a_aabb.height > b_pos.y)
            {
                println!("Peasant found campsite");
                // if player has enough gold
                commands.get_entity(a_entity).unwrap().remove::<FollowPlayer>();
                commands.get_entity(a_entity).unwrap().insert(ReadyForJob);
            }
    }
}

pub fn peasant_ready_for_job(input: Res<Input<KeyCode>>, mut commands: Commands, peasant_q: Query<(Entity, &Peasant, &Transform), With<ReadyForJob>>, mut writer: EventWriter<BindUi>, mut destroy: EventWriter<DestroyUi>, asset_server: Res<AssetServer>){

    if( peasant_q.is_empty()) { return; }
    if input.just_released(KeyCode::Q) == false { return ;}
    for (entity, _, transform) in peasant_q.iter(){
        let pos = transform.translation;
        spawn_builder(&mut commands, &asset_server, pos, &mut writer);

        // Should have something that handles the despawning of entity and UI rather than manually doing it everywhere
        destroy.send(DestroyUi(entity));
        commands.get_entity(entity).unwrap().despawn();
        return;
    }
}

#[derive(Component)]
pub struct ReadyForJob;

pub fn peasant_follow_player(
    mut minion_query: Query<(&mut Transform, &mut Velocity), (With<Peasant>, Without<Player>, With<FollowPlayer>)>,
    mut player_query: Query<&mut Transform, With<Player>>) 
{
    let player = player_query.single_mut();
    for mut minion in minion_query.iter_mut() {
        if minion.0.translation.distance(player.translation) <  150.0 { continue; }
        
        let direction = (player.translation - minion.0.translation).normalize();
        minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
}
