use bevy::{prelude::*, app::PluginGroupBuilder};

pub fn setup(mut commands: Commands){
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
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
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        },
    )
        .build()
}