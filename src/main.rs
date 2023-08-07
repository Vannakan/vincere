use bevy::prelude::*;
use vincere::{startup::*, window_icon::set_window_icon, MinionPlugin, PlayerPlugin, BuilderPlugin, CampsitePlugin, MapPlugin, scroll_events, TreePlugin, BanditPlugin, EntityUiPlugin, CombatPlugin, MovementPlugin};

fn main() {
    App::new()
    .add_plugins((build_default_plugins(), PlayerPlugin, MinionPlugin, BuilderPlugin, CampsitePlugin, MapPlugin, TreePlugin, BanditPlugin, EntityUiPlugin, CombatPlugin, MovementPlugin))
    .add_systems(Startup, setup)
    .add_systems(Startup, set_window_icon)
    .add_systems(Update, scroll_events)
    .run();
}