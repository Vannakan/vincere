use bevy::{input::mouse::MouseWheel, prelude::{EventReader, Query, OrthographicProjection, Without}};

use bevy::{prelude::*, render::{camera::Viewport, view::RenderLayers}, core_pipeline::clear_color::ClearColorConfig};

use crate::constants::{MINIMAP_WIDTH, WINDOW_WIDTH, WINDOW_HEIGHT, MINIMAP_HEIGHT, CAMERA_ZOOM};

pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
        .add_systems(Update, scroll_events);
    }
}

#[derive(Component)]
pub struct Minimap {}

// move cam to camera mod
pub fn setup_camera(mut commands: Commands){
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: 20.0,
            ..default()
        },
        camera_2d: Camera2d {
            // disable clearing completely (pixels stay as they are)
            // (preserves output from previous frame or camera/pass)
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            order:3,
            viewport: Some(Viewport {
                physical_position: UVec2::new((WINDOW_WIDTH - MINIMAP_WIDTH) as u32, (WINDOW_HEIGHT - MINIMAP_HEIGHT) as u32),
                physical_size: UVec2::new(MINIMAP_WIDTH as u32, MINIMAP_HEIGHT as u32),
                ..default()
            }),
            ..Default::default()
        },
        ..default()
    },
    RenderLayers::from_layers(&[1]),
    Minimap {},
    UiCameraConfig {
        show_ui: false,
    }));


    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_ZOOM,
            ..default()
        },
        camera: Camera {
            order: 2,
            ..Default::default()
        },
        ..default()
    },
    RenderLayers::from_layers(&[0,1])));
}

pub fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, Without<Minimap>>)
 {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let mut proj = query.single_mut();
                if ev.y < 0.0 {
                    proj.scale += 0.2;
                }
                else {
                    proj.scale -= 0.2;
                }
            }
            _ => { return; }
        }
    }
}