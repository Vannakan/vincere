use bevy::{input::mouse::MouseWheel, prelude::{EventReader, Query, OrthographicProjection, Without}};

use crate::Minimap;

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