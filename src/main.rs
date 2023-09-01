use bevy::prelude::*;
use vincere::{
    // startup::*, 
    // window_icon::set_window_icon,
    ui::plugin::EntityUiPlugin,
    combat::plugin::CombatPlugin,
    entities::plugin::EntitiesPlugin, DebugPlugin, game::plugin::GamePlugin, SetupPlugin,
};

fn main() {
    App::new()
    .add_plugins((
      //  build_default_plugins(),
        SetupPlugin,
        GamePlugin,
        EntitiesPlugin,
        EntityUiPlugin, 
        CombatPlugin, 

))
    .add_plugins(DebugPlugin)
    .run();
}
