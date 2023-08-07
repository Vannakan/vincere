use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{spawn_builder, BindUi, SpawnMinion, Player};

#[derive(Event)]
pub struct CreateCampsite;

#[derive(Resource)]
pub struct CampfireState(bool);

#[derive(Component)]
pub struct Campsite{
    pub trees: i32,
}

const STARTING_WOOD: i32 = 500;
const BUILDER_COST: i32 = 3;
const WORKER_COST: i32 = 3;

fn send_spawn_camp(input: Res<Input<KeyCode>>, mut writer: EventWriter<CreateCampsite>){
    if input.pressed(KeyCode::C) {
        writer.send(CreateCampsite);
    }
}

fn spawn_camp(
    mut commands:Commands, 
    mut evt: EventReader<CreateCampsite>,
    mut campfire_state: ResMut<CampfireState>,
    asset_server: Res<AssetServer>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,)
    {
        let (camera, camera_transform) = camera_q.single();
        if campfire_state.0 == true { return; }

        for _ in evt.iter() {
            if let Some(position) = q_windows.single().cursor_position(){

                let campfire_position =  Vec3::from((camera.viewport_to_world_2d(camera_transform, position).unwrap(), 1.0));
                spawn_campsite(&mut commands, campfire_position, &asset_server);                 
                campfire_state.0 = true;
            }
        }
}

pub struct CampsitePlugin;

impl Plugin for CampsitePlugin {
    fn build(&self, app: &mut App){
        app
        .insert_resource(CampfireState(false))
        .add_systems(Update, (send_spawn_camp, spawn_camp, try_spawn_minion, try_spawn_builder))
        .add_event::<CreateCampsite>();     
    }
}

fn try_spawn_builder(
    input: Res<Input<KeyCode>>,
    p_query: Query<&Transform,(Without<Camera>, With<Player>)>,
    mut c_query: Query<(&Transform, &mut Campsite),(Without<Player>, With<Campsite>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bind_ui: EventWriter<BindUi>)
{
    if input.just_released(KeyCode::Q) == false { return; }
    if p_query.is_empty() || c_query.is_empty() { return; }
    
    let player = p_query.single();
    let (campfire, mut state) = c_query.single_mut(); 

    if player.translation.distance(campfire.translation) < 150.0 && state.trees >= BUILDER_COST
    {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-100.0..=100.0);
        let y = rng.gen_range(-100.0..=100.0);
        spawn_builder(&mut commands, &asset_server, Vec3::from((campfire.translation.x - x, campfire.translation.y - y, campfire.translation.z)), &mut bind_ui);   
        state.trees -= BUILDER_COST;
    }
}

fn try_spawn_minion(
    input: Res<Input<KeyCode>>,
    p_query: Query<&Transform, (Without<Camera>, With<Player>)>,
    mut c_query: Query<(&Transform, &mut Campsite), (Without<Player>, With<Campsite>)>,
    mut writer: EventWriter<SpawnMinion>)
{
    if input.just_released(KeyCode::E) == false { return ;}
    let player = p_query.single();
    let (campfire, mut state) = c_query.single_mut(); 

    if player.translation.distance(campfire.translation) < 150.0 && state.trees >= WORKER_COST
    {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-100.0..=100.0);
        let y = rng.gen_range(-100.0..=100.0);
        writer.send(SpawnMinion(player.translation + Vec3{x, y, z: 1.0}));
        state.trees -= WORKER_COST;
    }
}

fn spawn_campsite(commands: &mut Commands, position: Vec3, asset_server: &Res<AssetServer>)
{
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(400.0, 400.0)),
            color: Color::ORANGE_RED,
            ..default()
        },
        transform: Transform{
            translation: position,
            ..default()
        },
        ..default()
        }, Campsite{
        trees: STARTING_WOOD
    },
));


commands.spawn(SpriteBundle {
    sprite: Sprite {
        custom_size: Some(Vec2::new(100.0, 100.0)),
        color: Color::BEIGE,
        ..default()
    },
    transform: Transform{
        translation: position,
        ..default()
    },
    texture: asset_server.load("fire.png"),
    ..default()
    });
}
