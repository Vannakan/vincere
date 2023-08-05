use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Tree;

pub fn spawn_tree(mut commands: &mut Commands, asset_server: &mut ResMut<AssetServer>, position: Vec3){
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
    }, Tree));
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, add_tree);
    }
}


fn add_tree(mut commands: Commands, mut asset_server: ResMut<AssetServer>, input: Res<Input<KeyCode>>, q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>){
        if let Some(position) = q_windows.single().cursor_position(){
        let (camera, camera_transform) = camera_q.single();
        if(input.just_released(KeyCode::T)){
            println!("jeioprgjoperg");
            spawn_tree(&mut commands, &mut asset_server, Vec3::from((camera.viewport_to_world_2d(camera_transform, position).unwrap(), 1.0)))
        }
    }
}