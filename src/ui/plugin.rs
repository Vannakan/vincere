use bevy::prelude::{Plugin, App, Startup, PostStartup, Update};

use super::systems::*;
use super::events::*;

pub struct EntityUiPlugin;

impl Plugin for EntityUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, create_ui_root)
        .add_systems(Startup, create_player_ui)
        .add_systems(PostStartup, create_tree_ui)
        .add_systems(PostStartup, create_gold_ui)
        .add_systems(Update, (entity_ui_movement, create_ui_binding))
        .add_systems(Update, destroy_ui)
        .add_systems(Update, on_attacked_ui_system)
        .add_systems(Update, update_tree_ui)
        .add_systems(Update, update_player_gold_ui)
        .add_event::<BindUi>()
        .add_event::<DestroyUi>()
        .add_event::<EntityAttacked>();
    }
}
