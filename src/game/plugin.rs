use bevy::prelude::Plugin;

use crate::{gold::GoldPlugin, camp::CampPlugin, tree::TreePlugin, campsite::CampsitePlugin};

use super::{movement::MovementPlugin, map::MapPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            GoldPlugin, 
            CampPlugin,
            TreePlugin,
            CampsitePlugin, 
            MovementPlugin,
            MapPlugin  ));
    }
}