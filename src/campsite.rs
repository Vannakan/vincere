use bevy::{prelude::*, window::PrimaryWindow};

use crate::spawn_builder;

#[derive(Event)]
pub struct CreateCampsite;

#[derive(Resource)]
pub struct CampfireState(bool);

#[derive(Component)]
pub struct Campsite{
    pub trees: i32,
}


fn send_spawn_camp(input: Res<Input<KeyCode>>, mut writer: EventWriter<CreateCampsite>){
    if(input.pressed(KeyCode::C)){
        writer.send(CreateCampsite);
    }
}

fn spawn_camp(
    mut commands:Commands, mut evt: EventReader<CreateCampsite>,
    mut campfire_state: ResMut<CampfireState>,
    asset_server: Res<AssetServer>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>){
    let (camera, camera_transform) = camera_q.single();
    if campfire_state.0 == true { return; }

    for ev in evt.iter() {
        if let Some(position) = q_windows.single().cursor_position(){

            let campfire_position =  Vec3::from((camera.viewport_to_world_2d(camera_transform, position).unwrap(), 1.0));
            spawn_campsite(&mut commands, campfire_position);        
            spawn_builder(&mut commands, &asset_server, Vec3::from((campfire_position.x - 100.0, campfire_position.y - 100.0, campfire_position.z)));   
            spawn_builder(&mut commands, &asset_server, Vec3::from((campfire_position.x + 100.0, campfire_position.y+ 100.0, campfire_position.z)));   
            spawn_builder(&mut commands, &asset_server, Vec3::from((campfire_position.x - 100.0, campfire_position.y + 100.0, campfire_position.z)));           
            spawn_builder(&mut commands, &asset_server, Vec3::from((campfire_position.x + 100.0, campfire_position.y - 100.0, campfire_position.z)));           
            campfire_state.0 = true;
        }
    }
}

pub struct CampsitePlugin;

impl Plugin for CampsitePlugin {
    fn build(&self, app: &mut App){
        app
        .insert_resource(CampfireState(false))
        .add_systems(Update, (send_spawn_camp, spawn_camp))
        .add_event::<CreateCampsite>();     
    }
}


fn spawn_campsite(commands: &mut Commands, position: Vec3){
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(400.0, 400.0)),
            color: Color::BEIGE,
            ..default()
        },
        transform: Transform{
            translation: position,
            ..default()
        },
        ..default()
    }, Campsite{
        trees: 0
    }));
}
