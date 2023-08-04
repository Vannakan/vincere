use bevy::prelude::*;
use vincere::{startup::*, window_icon::set_window_icon, map::*, MinionPlugin, PlayerPlugin, BuilderPlugin, CampsitePlugin, MapPlugin, scroll_events};

fn main() {
    App::new()
    .add_plugins((build_default_plugins(), PlayerPlugin, MinionPlugin, BuilderPlugin, CampsitePlugin, MapPlugin))
    .add_systems(Startup, setup)
    .add_systems(Startup, set_window_icon)
    .add_systems(Update, scroll_events)
    .run();
}