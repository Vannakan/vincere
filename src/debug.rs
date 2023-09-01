use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};


pub struct DebugPlugin;

impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default());
    }
}

