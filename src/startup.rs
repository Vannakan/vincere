use bevy::{prelude::*, app::PluginGroupBuilder};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());


    let texture = asset_server.load("player.png");
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
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