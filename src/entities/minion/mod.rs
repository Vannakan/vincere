use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::combat::components::AttackInfo;
use crate::combat::components::Attackable;
use crate::combat::components::Attacks;
use crate::combat::components::Damage;
use crate::combat::components::FindTarget;
use crate::combat::components::HasTarget;
use crate::combat::components::Health;
use crate::combat::events::AttackEvent;
use crate::common::components::FollowPlayer;
use crate::common::components::Targetable;
use crate::common::components::BoundingBox;
use crate::constants::WARRIOR_ATTACK_RANGE;
use crate::game::camera::Minimap;
use crate::player::components::Player;
use crate::combat::systems::find_target_with_targetable;
use crate::ui::components::HasUi;
use crate::ui::events::BindUi;
use crate::{ change_state, entities::bandit::components::Bandit};
use crate::common::components::Velocity;

#[derive(Component)]
pub struct Minion;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App){
        app      
         .add_systems(Update, minion_follow_player)
         .add_systems(Update, handle_move_to_bandit)
         .add_systems(Update, on_spawn_minions)
         .add_systems(Update, minion_found_target)
         .add_systems(Update, find_target_with_targetable::<Bandit>)
         .add_systems(Update, handle_attack::<Minion>)
         .add_systems(Update, default_minion)
         .add_systems(Update, stay_in_range::<Minion>)
         .add_systems(Update, enemy_defeated)
         .add_systems(Update, check_bandit_click)
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
            range: 75.0
       },
        Velocity(Vec3::default()))).id();
    
    
        writer.send(BindUi(entity, "Warrior".to_string()));
    }
}

const ENEMY_RANGE: f32 = 500.0;

pub fn minion_follow_player(
    mut minion_query: Query<(&mut Transform, &mut Velocity), (With<Minion>, Without<Player>, With<FollowPlayer>)>,
    mut player_query: Query<&mut Transform, With<Player>>) 
{
    if minion_query.is_empty() || player_query.is_empty() { return; }
    let player = player_query.single_mut();
    for mut minion in minion_query.iter_mut() {
        if minion.0.translation.distance(player.translation) <  150.0 { continue; }
        
        let direction = (player.translation - minion.0.translation).normalize();
        minion.1.0 += Vec3{x: direction.x * 25.0, y: direction.y * 25.0, z: 0.0}                    
    }
}

//Search for entities for a particular component.
//Send out an event when you've found it

fn minion_found_target(mut commands: Commands, query: Query<(Entity, &Minion), Added<HasTarget>>)
{
    if query.is_empty() { return; }
    for evt in query.iter(){
        if let Ok(entity) = query.get(evt.0){
            if let Some(mut e) = commands.get_entity(entity.0){
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

fn blank_minion_state(mut commands: &mut Commands, entity: Entity) {
    let mut e = commands.get_entity(entity).unwrap();
    e.remove::<Attack>();
    e.remove::<HasTarget>();
    e.remove::<MoveTo>();
    e.remove::<FindTarget::<Bandit>>();
    e.remove::<FollowPlayer>();
}

#[derive(Component)]
pub struct Attack;

#[derive(Event)]
pub struct TargetClicked<T: Component>(Entity, PhantomData<T>);

pub fn check_bandit_click(
    input: Res<Input<MouseButton>>,
    query: Query<(&BoundingBox, &Transform, Entity), With<Bandit>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), Without<Minimap>>,
    m_query: Query<Entity, With<Minion>>,
    mut commands: Commands){

    if query.is_empty() { return; }
    if m_query.is_empty() { return; }
    if(input.just_released(MouseButton::Left)){
        let (camera, camera_transform) = camera_q.single();
        let mousePosition =  q_windows.single().cursor_position().unwrap();
        let position =Vec3::from((camera.viewport_to_world_2d(camera_transform, mousePosition).unwrap(), 1.0));

        for entity in query.iter(){
            let a_pos = entity.1.translation;
            let a_aabb = entity.0;
            let b_pos = position;
            let b_aabb = BoundingBox{
                height: 5.0,
                width: 5.0
            };

            if a_aabb.intersects(a_pos, &b_aabb, b_pos) == false { continue; }

            // TODO: IF FINDS CLOSER TARGET, ATTACK THAT INSTEAD

            for minion in m_query.iter(){
                blank_minion_state(&mut commands, minion);
                change_state::<FollowPlayer>(&mut commands, minion, HasTarget{target: Some(entity.2)}) 
            }
        }
    }
}


// maybe bands/groups move around in a "flock". Flock being a radius that if
//you click into, warriors will "seek" to the flock and attack the closest enemy to them
//if there are X warriors on a bandit
//find another bandit nearby
//if there are no other bandits
//attack that bandit


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
                
        if transform.translation.distance(bandit.1.translation) <  WARRIOR_ATTACK_RANGE
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