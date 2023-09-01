use bevy::{prelude::*, render::view::RenderLayers};
use rand::{random, Rng};
use noisy_bevy::{fbm_simplex_2d_seeded};

use crate::{ui::events::BindUi, rabbit::spawn_rabbit_2, game::{camp::spawn_camp_2, tree::Tree}};

const SMOOTHNESS: f32 = 0.75; //0.75 is a good one
const TILE_SIZE:f32 = 30.0; //50
const MAP_WIDTH:i32 = 50;
const MAP_HEIGHT:i32 = 50;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, (map_regen_pressed, map_regen_pressed_delete_trees));
    }
}

#[derive(Component)]
pub struct Tile;

pub fn map_regen_pressed(mut commands: Commands, keycode: Res<Input<KeyCode>>, query:Query<Entity, &Tile>, asset_server: Res<AssetServer>, mut writer: EventWriter<BindUi>){

    if keycode.just_released(KeyCode::R){
        regen_map(&mut commands, query, &asset_server, &mut writer);
    }
}

pub fn map_regen_pressed_delete_trees(mut commands: Commands, keycode: Res<Input<KeyCode>>, query:Query<Entity, &Tree>, mut asset_server: ResMut<AssetServer>){

    if keycode.just_released(KeyCode::R){
        for q in query.iter() {
            commands.entity(q).despawn();
        }
    }
}

fn regen_map(mut commands: &mut Commands, query:Query<Entity, &Tile>, asset_server: &Res<AssetServer>, mut writer: &mut EventWriter<BindUi>){
    
    for entity in query.iter(){
        commands.entity(entity).despawn();
    }

    map(&mut commands, &asset_server, &mut writer)
}

fn spawn_tile_2(commands: &mut Commands, texture: Handle<Image>, x: f32, y:f32, z:f32, color: Color){
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: color,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
     //   texture,          
        transform: Transform { translation: Vec3 { x, y, z }, ..default() },
        ..default()
    },
    Tile,
    RenderLayers::layer(1)));
}




pub fn add_camps(mut commands: &mut Commands, asset_server: &Res<AssetServer>, mut writer:  &mut EventWriter<BindUi>) {
    let mut rng =  rand::thread_rng();
    let campsites_to_spawn = rng.gen_range(3..6);
    let mut campsite_coords: Vec<Vec2> = Vec::new();


    for num in 0..=campsites_to_spawn {
        let x = rng.gen_range(0.0..MAP_WIDTH as f32);
        let y = rng.gen_range(0.0..MAP_HEIGHT as f32);
        let spawn_pos = Vec2::from([x * 30.0,y * 30.0]);
        //if is_on_land (spawn_pos)
        spawn_camp_2(&mut commands, asset_server, writer, spawn_pos);
    }
}

const OCTAVES: usize = 7;
const LACUNARITY: f32 = 2.0;
const GAIN: f32 = 0.5;

// https://www.reddit.com/r/gamedev/comments/1g4eae/need_help_generating_an_island_using_perlin_noise/
//The distance formula is: distance = sqrt( (x2 - x1)2 + (y2 - y1)2 )
pub fn map(mut commands: &mut Commands, asset_server: &Res<AssetServer>, mut writer: &mut EventWriter<BindUi>){

    let seed = random::<f32>();
    let mut rand = rand::thread_rng();
    let mut y_off = 0.0;
    for x in -MAP_WIDTH..=MAP_WIDTH {
        let mut x_off = 0.0;
        for y in -MAP_WIDTH..=MAP_HEIGHT {
            // Distance from center
            let center_x =  x * x;
            let center_y =  y * y;
            let result = f32::sqrt(center_x as f32 + center_y as f32) / (MAP_WIDTH*2) as f32;
            // Fractal Noise 
            let noise =  fbm_simplex_2d_seeded(Vec2{x :x_off  , y: y_off  }, OCTAVES, LACUNARITY, GAIN, seed);

             if noise - result > 0.5 {
                if rand.gen_range(0.0..=100.0) > 99.5 {
                    spawn_rabbit_2(&mut commands, asset_server, x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)
                }
                spawn_tile_2(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0, Color::GRAY);
            }
            else if noise - result > 0.2 {
                spawn_tile_2(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0, Color::hex("#6ca40c").unwrap());
            }
            else if noise - result > 0.0 {
                spawn_tile_2(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0, Color::hex("#964B00").unwrap());
            }
            
             else if noise - result > -0.1 {
                spawn_tile_2(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0, Color::YELLOW);
            }  
                           
             else if noise - result > -0.2 {
                spawn_tile_2(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0, Color::hex("#1c7cd4").unwrap());
            }      
            else {
                spawn_tile_2(&mut commands, asset_server.load("grass.png"), x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0, Color::hex("#0c3464").unwrap())
            }
            
            x_off+=0.01;
        }
        y_off+=0.01;
    }

    //add_camps(commands, asset_server, &mut writer)
}