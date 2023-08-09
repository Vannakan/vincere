
use std::{marker::PhantomData, borrow::BorrowMut};

use bevy::prelude::*;

use crate::{ player::Player, Velocity, change_state, get_nearest_entity, BindUi, HasUi, AttackEvent, Health, Damage, Attacks, components::Bandit };

#[derive(Component)]
pub struct Minion;

#[derive(Component)]
pub struct FollowPlayer;

#[derive(Component)]
pub struct Target{
    pub entity: Entity
}

#[derive(Component)]
pub struct Attackable;


#[derive(Component)]
pub struct AttackInfo{
    pub range: f32,
    pub cooldown: f32,
    pub last_attacked: f32
}

#[derive(Component)]
pub struct Targetable;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App){
        app      
         .add_systems(Update, minion_follow_player)
         .add_systems(Update, handle_move_to_bandit)
         .add_systems(Update, on_spawn_minions)
         .add_systems(Update, minion_found_target)
         .add_systems(Update, find_target::<Bandit>)
         .add_systems(Update, handle_attack::<Minion>)
         .add_systems(Update, default_minion)
         .add_systems(Update, stay_in_range::<Minion>)
         .add_systems(Update, enemy_defeated)
         .add_event::<SpawnMinion>();
    }
} 

#[derive(Event)]
pub struct SpawnMinion(pub Vec3);

fn on_spawn_minions(mut commands: Commands, asset_server: Res<AssetServer>, mut writer: EventWriter<BindUi>, mut reader:EventReader<SpawnMinion>){
    for evt in  reader.iter(){
        let texture = asset_server.load("knight.png");
        let entity = commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            transform: Transform{
                translation: evt.0,
                ..default()
            },
            ..default()
        }, 
        Health{
            starting: 30.0,
            current: 30.0
        },
        Damage(5.0),
        Minion,
        FollowPlayer,
        HasUi,
        Attackable,
        Targetable,
        FindTarget::<Bandit> {
            phantom: PhantomData
        },
        Attacks {
             last_attacked: 0.0
        },
        AttackInfo {
            last_attacked: 0.0,
            cooldown: 1.5,
            range: 200.0
       },
        Velocity(Vec3::default()))).id();
    
    
        writer.send(BindUi(entity, "Knight".to_string()));
    }
}

const ENEMY_RANGE: f32 = 500.0;

pub fn minion_follow_player(
    mut minion_query: Query<(&mut Transform, &mut Velocity), (With<Minion>, Without<Player>, With<FollowPlayer>)>,
    mut player_query: Query<&mut Transform, With<Player>>) 
{
    let player = player_query.single_mut();
    for mut minion in minion_query.iter_mut() {
        if minion.0.translation.distance(player.translation) <  150.0 { continue; }
        
        let direction = (player.translation - minion.0.translation).normalize();
        minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
}




//Search for entities for a particular component.
//Send out an event when you've found it

#[derive(Component)]
pub struct FindTarget<T: Component>{
    pub phantom: PhantomData<T>
}

pub fn find_target<TWith: Component>(
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
       // e.insert(FoundTarget::<TWith> { me: entity.0, to: nearest.0, phantom: PhantomData });
        e.insert(HasTarget{ target: Some(nearest.0)});
        e.remove::<FindTarget<TWith>>();

       // commands.entity(entity.0).insert((FoundTarget::<TWith> { me: entity.0, to: nearest.0, phantom: PhantomData }));
    }           
} 

#[derive(Component)]
pub struct FoundTarget<T: Component>{
    pub me: Entity,
    pub to: Entity,
    phantom: PhantomData<T>
}

#[derive(Component)]
pub struct HasTarget{
    pub target: Option<Entity>,
}

// fn minion_found_target(mut commands: Commands, query: Query<(Entity, &Minion, &FoundTarget<Bandit>), Added<FoundTarget<Bandit>>>)
// {
//     if query.is_empty() { return; }
//     for evt in query.iter(){
//         if let Ok(entity) = query.get(evt.0){
//             if let Some(mut e) = commands.get_entity(entity.0){

//                 e.insert(HasTarget{ target: Some(evt.2.to)});
//                 e.remove::<FoundTarget<Bandit>>();
//                 change_state::<FollowPlayer>(&mut commands, entity.0, MoveTo)
//             }
//         }
//     }
// }

fn minion_found_target(mut commands: Commands, query: Query<(Entity, &Minion), Added<HasTarget>>)
{
    if query.is_empty() { return; }
    for evt in query.iter(){
        if let Ok(entity) = query.get(evt.0){
            if let Some(mut e) = commands.get_entity(entity.0){

                // e.insert(HasTarget{ target: Some(evt.2.to)});
                // e.remove::<FoundTarget<Bandit>>();
                change_state::<FollowPlayer>(&mut commands, entity.0, MoveTo)
            }
        }
    }
}

pub fn enemy_defeated(mut commands: Commands, query: Query<(Entity, &HasTarget), With<Minion>>){
    for entity in query.iter(){
        let target = match(entity.1.target){
            Some(t) => t,
            _ => {
                reset_minion_state(&mut commands, entity.0);
                continue;
            }
        };

        match commands.get_entity(target) {
            Some(_) => continue,
            _ => {
                reset_minion_state(&mut commands, entity.0);
                continue;
            }
        };
    }
}

fn reset_minion_state(mut commands: &mut Commands, entity: Entity){
    let mut e = commands.get_entity(entity).unwrap();
    e.remove::<Attack>();
    e.remove::<HasTarget>();
    e.remove::<MoveTo>();
    e.insert(    FindTarget::<Bandit> {
        phantom: PhantomData
    });
    e.insert(FollowPlayer);
}

#[derive(Component)]
pub struct Attack;

pub fn handle_move_to_bandit(
    mut commands: Commands,
    mut minion_query: Query<(Entity, &mut Transform, &mut Velocity, &MoveTo, &HasTarget), With<Minion>>,
    bandit_query: Query<(Entity, &Transform), (With<Bandit>, Without<Minion>)>,)
{
    for (entity, transform, mut velocity, move_to, target) in minion_query.iter_mut()
    {
        let bandit = match bandit_query.get(target.target.unwrap()) 
        {
            Ok(b) => b,
            _ => {
                reset_minion_state(&mut commands, entity);
                continue
            } // Should be set option to none
        };
                
        if transform.translation.distance(bandit.1.translation) <  75.0
        { 
            change_state::<MoveTo>(&mut commands, entity, Attack);  // shouldnt dereference here
            continue;
        }

        let direction = (bandit.1.translation - transform.translation).normalize();
        velocity.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0} ;
    }
}

pub fn stay_in_range<TAttacker: Component>
(
    mut commands: Commands,
    mut mover_query: Query<(Entity, &mut AttackInfo, &Transform, &HasTarget), (With<TAttacker>, With<Attack>)>,
    target_query: Query<(Entity, &Transform), (With<Targetable>, Without<TAttacker>)>)
{
    for mover in mover_query.iter_mut()
    {
        // Do we have a target
        let target = match mover.3.target {
            Some(entity) => entity,
            _ => { 
                reset_minion_state(&mut commands, mover.0);
                continue;
             },
        };

        // Is that target still valid
        let entity = match target_query.get(target) {
            Ok(e) => e,
            _ => {                  
                reset_minion_state(&mut commands, mover.0);
                continue;
            }, 
        };
        
        if mover.2.translation.distance(entity.1.translation) > mover.1.range
        {
            let mut a = commands.get_entity(mover.0).unwrap();
            a.insert(MoveTo);
        }
    }
}
pub enum Test {
    TEST1,
    TEST2
}

#[derive(Component)]
pub struct MoveTo;


pub fn default_minion(mut commands: Commands, query: Query<Entity, (With<Minion>, Without<MoveTo>, Without<HasTarget>, Without<FollowPlayer>, Without<Attack>, Without<MoveTo>)>){
    if query.is_empty() { return; }
    for minion in query.iter(){
        reset_minion_state(&mut commands, minion);
    }
}


pub fn handle_attack<TAttacker: Component>
(
    mut commands: Commands,
    mut attacker_query: Query<(Entity, &mut AttackInfo, &Transform, &HasTarget), (With<TAttacker>, With<Attack>)>,
    attackable_query: Query<(Entity, &Transform), (With<Attackable>, Without<TAttacker>)>,
    time: Res<Time>,
    mut writer: EventWriter<AttackEvent>)
{
    for mut attacker in attacker_query.iter_mut()
    {
        if attacker.1.last_attacked > time.elapsed_seconds() - attacker.1.cooldown { continue; }

        // Do we have a target
        let target = match attacker.3.target {
            Some(entity) => entity,
            _ => { 
                reset_minion_state(&mut commands, attacker.0);
                continue;
             },
        };

        // Is that target still valid
        let entity = match attackable_query.get(target) {
            Ok(e) => e,
            _ => {                  
                reset_minion_state(&mut commands, attacker.0);
                continue;
            }, // Target doesnt exist, dont check next time
        };
        
        if attacker.2.translation.distance(entity.1.translation) < attacker.1.range
        {
            // Attack the entity
            writer.send(AttackEvent { from: attacker.2.clone(), to: entity.0, damage: 3.0});
            attacker.1.last_attacked = time.elapsed_seconds();
        }
    }
}
