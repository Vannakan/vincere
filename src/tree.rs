use bevy::{prelude::*, window::PrimaryWindow};

use crate::Minimap;

#[derive(Component)]
pub struct Tree;
#[derive(Component)]
pub struct Prop;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, add_tree);
    }
}

pub fn spawn_tree(commands: &mut Commands, asset_server: &mut ResMut<AssetServer>, position: Vec3){
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::BEIGE,
            ..default()
        },
        texture: asset_server.load("tree.png"),
        transform: Transform{
            translation:position,
            ..default()
        },
        ..default()
    }, Tree,
    Prop));
}

fn add_tree(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    input: Res<Input<KeyCode>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform),  Without<Minimap>>)
    {
        if let Some(position) = q_windows.single().cursor_position(){
        let (camera, camera_transform) = camera_q.single();
        if input.just_released(KeyCode::T)
        {
            spawn_tree(&mut commands, &mut asset_server, Vec3::from((camera.viewport_to_world_2d(camera_transform, position).unwrap(), 1.0)))
        }
    }
}