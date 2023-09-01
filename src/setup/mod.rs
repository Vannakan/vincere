use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy::winit::WinitWindows;
use bevy::window::PrimaryWindow;
use winit::window::Icon;


use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(( build_default_plugins())).add_systems(Startup, set_window_icon);
    }
}
pub fn build_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "vincere".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        },
    )
        .build()
}

// https://stackoverflow.com/questions/74586997/how-to-add-a-window-icon-in-bevy
pub fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {return};

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/window_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}