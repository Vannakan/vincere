use bevy::prelude::*;
use vincere::{
    startup::*, 
    window_icon::set_window_icon,
    ui::plugin::EntityUiPlugin,
    player::plugin::PlayerPlugin,
    combat::plugin::CombatPlugin,
    entities::plugin::EntitiesPlugin, DebugPlugin, game::{plugin::GamePlugin, camera::{scroll_events, setup}},
};

fn main() {
    App::new()
    .add_plugins((
        build_default_plugins(),
        EntitiesPlugin,
        GamePlugin,
        PlayerPlugin,
        EntityUiPlugin, 
        CombatPlugin, 
))
    .add_systems(Startup, setup)
    .add_systems(Startup, set_window_icon)
    .add_systems(Update, scroll_events)
    .add_plugins(DebugPlugin)
    .run();
}
