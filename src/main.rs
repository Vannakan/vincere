use bevy::prelude::*;
use vincere::{startup::*, window_icon::set_window_icon, map::*, MinionPlugin, PlayerPlugin};


fn main() {
    App::new()
    .add_plugins((build_default_plugins(), PlayerPlugin, MinionPlugin))
    .add_systems(Startup, setup)
    .add_systems(Startup, map)
    .add_systems(Startup, set_window_icon)
    .run();
}
