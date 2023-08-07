use bevy::prelude::*;
use rand::Rng;

use crate::{DestroyUi, EntityAttacked, Velocity};

#[derive(Event)]
pub struct Attack
{
  pub from:Transform,
  pub to: Entity,
  pub damage: f32   
}


#[derive(Component)]
pub struct Attacks {
    pub last_attacked: f32
}

#[derive(Component)]
pub struct Health{
    pub starting: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct Damage(pub f32);

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack_system, kill, push_back, play_attack_sound))
        .add_event::<Attack>()
        .add_event::<PushBack>();
    }
}

#[derive(Event)]
pub struct PushBack{
    pub from: Transform,
    pub to: Entity,
}

fn attack_system(
    mut events: EventReader<Attack>, 
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

fn play_attack_sound( 
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

fn push_back(mut events: EventReader<PushBack>, mut query: Query<(Entity, &mut Transform, &mut Velocity)>)
{
    if query.is_empty() { return;}

    for evt in events.iter(){
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
        to.2.0 -= Vec3::from((direction.x * 500.0, direction.y * 500.0, 0.0));
    }
}


fn kill(mut commands: Commands, query: Query<(Entity, &Health)>, mut writer: EventWriter<DestroyUi>)
{
    for entity in query.iter()
    {
        if entity.1.current <= 0.0 
        {
            writer.send(DestroyUi(entity.0));
            commands.entity(entity.0).despawn();
        }
    }
}
