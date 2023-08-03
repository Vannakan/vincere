use bevy::{prelude::*, app::PluginGroupBuilder};

const CAMERA_ZOOM: f32 = 5.0;
const WINDOW_HEIGHT: f32 = 1600.0;
const WINDOW_WIDTH: f32 = 1200.0;

pub fn setup(mut commands: Commands){
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_ZOOM,
            ..default()
        },
        ..default()
    });
}

pub fn build_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "vincere".to_string(),
                resolution: (WINDOW_HEIGHT, WINDOW_WIDTH).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        },
    )
        .build()
}