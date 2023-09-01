use bevy::{prelude::*, app::PluginGroupBuilder, render::{camera::Viewport, view::RenderLayers}, core_pipeline::clear_color::ClearColorConfig};

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

// const CAMERA_ZOOM: f32 = 2.0;
// const WINDOW_HEIGHT: f32 = 1200.0;
// const WINDOW_WIDTH: f32 = 1600.0;
// const MINIMAP_HEIGHT: f32 = 256.0;
// const MINIMAP_WIDTH: f32 = 256.0;

// #[derive(Component)]
// pub struct Minimap {}

// // move cam to camera mod
// pub fn setup(mut commands: Commands){
//     commands.spawn((Camera2dBundle {
//         projection: OrthographicProjection {
//             scale: 20.0,
//             ..default()
//         },
//         camera_2d: Camera2d {
//             // disable clearing completely (pixels stay as they are)
//             // (preserves output from previous frame or camera/pass)
//             clear_color: ClearColorConfig::None,
//         },
//         camera: Camera {
//             order:3,
//             viewport: Some(Viewport {
//                 physical_position: UVec2::new((WINDOW_WIDTH - MINIMAP_WIDTH) as u32, (WINDOW_HEIGHT - MINIMAP_HEIGHT) as u32),
//                 physical_size: UVec2::new(MINIMAP_WIDTH as u32, MINIMAP_HEIGHT as u32),
//                 ..default()
//             }),
//             ..Default::default()
//         },
//         ..default()
//     },
//     RenderLayers::from_layers(&[1]),
//     Minimap {},
//     UiCameraConfig {
//         show_ui: false,
//     }));


//     commands.spawn((Camera2dBundle {
//         projection: OrthographicProjection {
//             scale: CAMERA_ZOOM,
//             ..default()
//         },
//         camera: Camera {
//             order: 2,
//             ..Default::default()
//         },
//         ..default()
//     },
//     RenderLayers::from_layers(&[0,1])));
// }

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