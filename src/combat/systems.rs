use bevy::prelude::*;
use rand::Rng;

use crate::{get_nearest_entity, ui::events::{EntityAttacked, DestroyUi}, common::components::{BoundingBox, Velocity, Targetable}, game::gold::{Inventory, GoldCoin, Coin}};

use super::{components::{FindTarget, HasTarget, Health}, events::{AttackEvent, PushBack}};

const ENEMY_RANGE: f32 = 500.0;

// Should ahve a range component
pub fn global_find_target_with<TWith: Component>(
    mut commands: Commands,
    query: Query<(Entity, &mut Transform), (With<FindTarget<TWith>>, Without<TWith>, Without<HasTarget>)>,
    mut target_query: Query<(Entity, &Transform), (With<TWith>)>,
    )
{
    if target_query.is_empty() || query.is_empty()  { return; }
    let mut targets  = target_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for entity in query.iter(){
        let nearest = get_nearest_entity(&mut targets, entity.1.translation);
        println!("FOUND TARGET");
        let mut e = commands.entity(entity.0);
        e.insert(HasTarget{ target: Some(nearest.0)});
        e.remove::<FindTarget<TWith>>();
    }           
} 


// Should ahve a range component
pub fn find_target_with<TWith: Component>(
    mut commands: Commands,
    query: Query<(Entity, &mut Transform), (With<FindTarget<TWith>>, Without<TWith>, Without<HasTarget>)>,
    mut target_query: Query<(Entity, &Transform), (With<TWith>)>,
    )
{
    if target_query.is_empty() || query.is_empty()  { return; }
    let mut targets  = target_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for entity in query.iter(){
        let nearest = get_nearest_entity(&mut targets, entity.1.translation);
        if nearest.1.translation.distance(entity.1.translation) > ENEMY_RANGE {
            continue;
        } 

        println!("FOUND TARGET");
        let mut e = commands.entity(entity.0);
        e.insert(HasTarget{ target: Some(nearest.0)});
        e.remove::<FindTarget<TWith>>();
    }           
} 

pub fn find_target_with_targetable<TWith: Component>(
    mut commands: Commands,
    query: Query<(Entity, &mut Transform), (With<FindTarget<TWith>>, Without<TWith>, Without<HasTarget>)>,
    mut target_query: Query<(Entity, &Transform), (With<TWith>, With<Targetable>)>,
    )
{
    if target_query.is_empty() || query.is_empty()  { return; }
    let mut targets  = target_query.iter_mut().collect::<Vec<(Entity, &Transform)>>();
    for entity in query.iter(){
        let nearest = get_nearest_entity(&mut targets, entity.1.translation);
        if nearest.1.translation.distance(entity.1.translation) > ENEMY_RANGE {
            continue;
        } 
        println!("bandit found a target");
        let mut e = commands.entity(entity.0);
        e.insert(HasTarget{ target: Some(nearest.0)});
        e.remove::<FindTarget<TWith>>();
    }           
} 


pub fn attack_system(
    mut events: EventReader<AttackEvent>, 
    mut query: Query<(Entity, &mut Health)>,
    mut attacked_writer: EventWriter<EntityAttacked>,
    mut push_writer: EventWriter<PushBack>)
{
    for evt in events.iter(){
        let e = query.iter_mut().find(|(x, _y)| x == &evt.to
        );

        if let Some(mut e) = e {
            e.1.current -= evt.damage;

            attacked_writer.send(EntityAttacked { entity: e.0, health_left: e.1.current, starting_health: e.1.starting });
            push_writer.send(PushBack{from: evt.from, to: evt.to});
        }
    }
}


pub fn play_attack_sound( 
    mut commands: Commands,
    mut events: EventReader<EntityAttacked>,
    asset_server: Res<AssetServer>
)
{
    let mut rng = rand::thread_rng();
    for _ in events.iter()
    {
        let audio = asset_server.load(format!("audio/hit-{}.ogg", rng.gen_range(0..=4)));
        commands.spawn(AudioBundle{
            source: audio,
            settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn, ..Default::default() },
        });
    }
}


const KNOCKBACK: f32 = 1000.0;

pub fn push_back(mut events: EventReader<PushBack>, mut query: Query<(Entity, &mut Transform, &mut Velocity)>)
{
    if query.is_empty() { return;}

    for evt in events.iter()
    {
        let from = evt.from;

        let mut to = {
            let push = match query
            .iter_mut()
            .find(|e| e.0 == evt.to
            ){
                Some(entity_to_push) => entity_to_push,
                None => continue
            };
            push
        };

        let direction = (from.translation - to.1.translation).normalize();
        to.2.0 -= Vec3::from((direction.x * KNOCKBACK, direction.y * KNOCKBACK, 0.0));
    }
}

pub fn kill_inventory(mut commands: Commands, query: Query<(Entity, &Health, &Inventory, &Transform)>, mut writer: EventWriter<DestroyUi>, asset_server: Res<AssetServer>)
{
    if query.is_empty() { return;}
    for entity in query.iter()
    {
        if entity.1.current <= 0.0 
        {
            writer.send(DestroyUi(entity.0));
             // CAN FIX THE ISSUE BY HAVING A SYSTEM THAT RUNS LAST THAT RESPAWNS ENTITIES MARKED TO BE DESPAWNED run this last
            commands.entity(entity.0).despawn();
            commands.spawn(AudioBundle{
                source: asset_server.load("audio/death.ogg"),
                settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn, ..Default::default() }
            });

            let mut rng = rand::thread_rng();
            for num in 0..=entity.2.coins {
                let x_offset = rng.gen_range(25.0..50.0);
                let y_offset = rng.gen_range(25.0..50.0);
                commands.spawn((GoldCoin{
                    bounding_box: BoundingBox { width: 32.0, height: 32.0 },
                    coin: Coin {
                        amount: 1
                    },
                    sprite: SpriteBundle { transform: Transform { translation: Vec3::from([entity.3.translation.x + x_offset, entity.3.translation.y + y_offset, 1.0]), ..default() }
                        , texture: asset_server.load("gold.png"), ..Default::default()}
                }));
            }
        }
    }
}

pub fn kill(mut commands: Commands, query: Query<(Entity, &Health), Without<Inventory>>, mut writer: EventWriter<DestroyUi>, asset_server: Res<AssetServer>)
{
    for entity in query.iter()
    {
        if entity.1.current <= 0.0 
        {
            writer.send(DestroyUi(entity.0));
             // CAN FIX THE ISSUE BY HAVING A SYSTEM THAT RUNS LAST THAT RESPAWNS ENTITIES MARKED TO BE DESPAWNED run this last
            commands.entity(entity.0).despawn();
            commands.spawn(AudioBundle{
                source: asset_server.load("audio/death.ogg"),
                settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn, ..Default::default() }
            });
        }
    }
}
