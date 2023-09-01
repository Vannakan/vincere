use bevy::prelude::Plugin;

use crate::{builder::BuilderPlugin, rabbit::RabbitPlugin, bandit::BanditPlugin};

use super::{peasant::PeasantPlugin, minion::MinionPlugin};

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((    
            MinionPlugin,
            BuilderPlugin,
            BanditPlugin, 
            RabbitPlugin,
            PeasantPlugin, ));
    }
}