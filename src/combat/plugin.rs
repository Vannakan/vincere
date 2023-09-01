use bevy::prelude::*;

use crate::combat::systems::{kill_inventory, kill};

use super::systems::{attack_system, push_back, play_attack_sound};
use super::events::{AttackEvent, PushBack};
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack_system, push_back, play_attack_sound))
        .add_event::<AttackEvent>()
        .add_event::<PushBack>()
        .add_systems(PostUpdate, (kill, kill_inventory)); //despawn logic;
    }
}