use bevy::prelude::*;
mod components;
mod tree;
mod campsite;
use crate::builder::campsite::*;
use crate::builder::components::*;
use crate::builder::tree::*;
use crate::{Velocity, HasUi, BindUi};

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, (
            find_tree,
            cut_tree, 
            move_to_tree,
            move_to_campsite,
            deposit_campsite,
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
        last_action: 0.0
    },
    FindTree,
    HasUi,
    Velocity(Vec3::default()))).id();

    writer.send(BindUi(entity, "Builder".to_string()));
}
