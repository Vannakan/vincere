#[cfg(target_arch = "wasm32")]
console_error_panic_hook::set_once!();

use bevy::prelude::*;
use vincere::{
    startup::*, 
    window_icon::set_window_icon,
    MinionPlugin,
    CampsitePlugin,
    MapPlugin,
    scroll_events,
    TreePlugin, 
    MovementPlugin,
    GoldPlugin, 
    PeasantPlugin,
    CampPlugin,
    ui::plugin::EntityUiPlugin,
    player::plugin::PlayerPlugin, 
    combat::systems::{kill_inventory, kill},
    combat::plugin::CombatPlugin, 
    entities::bandit::BanditPlugin,
    entities::builder::BuilderPlugin,
};

fn main() {
    App::new()
    .add_plugins((build_default_plugins(),
        PlayerPlugin,
        MinionPlugin,
        BuilderPlugin,
        CampsitePlugin, 
        MapPlugin,
        TreePlugin, 
        BanditPlugin, 
        EntityUiPlugin, 
        CombatPlugin, 
        MovementPlugin, 
        GoldPlugin, 
        PeasantPlugin, 
        CampPlugin)
    )
    .add_systems(Startup, setup)
    .add_systems(Startup, set_window_icon)
    .add_systems(Update, scroll_events)
    .add_systems(PostUpdate, (kill, kill_inventory)) //despawn logic
    .run();
}