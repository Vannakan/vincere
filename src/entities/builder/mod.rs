use std::marker::PhantomData;

use bevy::prelude::*;
mod components;
mod tree;
mod campsite;
use crate::Tree;
use self::components::*;
use self::tree::*;
use crate::combat::components::FindTarget;
use crate::common::components::Velocity;
use crate::combat::systems::global_find_target_with;
use crate::ui::components::HasUi;
use crate::ui::events::BindUi;

use self::campsite::deposit_campsite;
use self::campsite::find_campsite;
use self::campsite::move_to_campsite;

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, (
           // find_tree,
            cut_tree, 
            move_to_tree,
            find_campsite,
           move_to_campsite,
           deposit_campsite,
           global_find_target_with::<Tree>,
            builder_found_target,
    ));}
}

pub fn spawn_builder(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3, writer: &mut EventWriter<BindUi>){
    let texture = asset_server.load("builder.png");
    let entity = commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        transform: Transform{
            translation: position,
            ..default()
        },
        ..default()
    }, 
    Builder{
        trees: 0,
    },
    Action {
        last_action: 0.0
    },
    HasUi,
    FindTarget::<Tree>{
        phantom:PhantomData
    },
    Velocity(Vec3::default()))).id();

    writer.send(BindUi(entity, "Builder".to_string()));
}
