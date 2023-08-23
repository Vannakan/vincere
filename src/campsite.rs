use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::builder::spawn_builder;
use crate::player::components::Player;
use crate::ui::events::BindUi;
use crate::{SpawnMinion, Inventory, Minimap};

use crate::common::components::BoundingBox;

#[derive(Event)]
pub struct CreateCampsite;

#[derive(Resource)]
pub struct CampfireState(bool);

#[derive(Component)]
pub struct Campsite{
    pub trees: i32,
}

const STARTING_WOOD: i32 = 0;
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
    camera_q: Query<(&Camera, &GlobalTransform), Without<Minimap>>,)
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bind_ui: EventWriter<BindUi>)
{
    if input.just_released(KeyCode::Key1) == false { return; }
    if p_query.is_empty() { return; }
    
    let player = p_query.single();

    // if player.translation.distance(campfire.translation) < 150.0 && state.trees >= BUILDER_COST
    // {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-100.0..=100.0);
        let y = rng.gen_range(-100.0..=100.0);
        spawn_builder(&mut commands, &asset_server, Vec3::from((player.translation.x - x, player.translation.y - y, player.translation.z)), &mut bind_ui);   
   // }
}

fn try_spawn_minion(
    input: Res<Input<KeyCode>>,
    mut p_query: Query<(&Transform, &mut Inventory), (Without<Camera>, With<Player>)>,
    mut c_query: Query<(&Transform, &mut Campsite), (Without<Player>, With<Campsite>)>,
    mut writer: EventWriter<SpawnMinion>)
{
    if p_query.is_empty() || c_query.is_empty() { return; }
    if input.just_released(KeyCode::E) == false { return ;}
    let (transform, mut inventory) = p_query.single_mut();
    let (campfire, mut state) = c_query.single_mut(); 

    if transform.translation.distance(campfire.translation) < 150.0 && inventory.coins >= WORKER_COST
    {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-100.0..=100.0);
        let y = rng.gen_range(-100.0..=100.0);
        writer.send(SpawnMinion(transform.translation + Vec3{x, y, z: 1.0}));
        inventory.coins -= WORKER_COST;
    }
}

fn spawn_campsite(commands: &mut Commands, position: Vec3, asset_server: &Res<AssetServer>)
{
    commands.spawn((SpriteBundle {
        sprite: Sprite {
             custom_size: Some(Vec2::new(150.0, 150.0)),
            // color: Color::ORANGE_RED,
            ..default()
        },
        texture: asset_server.load("fire.png"),
        transform: Transform{
            translation: position,
            ..default()
        },
        ..default()
        },
        BoundingBox{
            width:  250.0,
            height: 250.0
        } ,
        Campsite{
        trees: STARTING_WOOD
    },
));
}
