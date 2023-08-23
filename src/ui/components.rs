use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct UiTransformBinding {
    pub ui_entity: Entity,
    pub y_offset: f32,
}

#[derive(Component)]
pub struct HasUi;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct EntityUiRoot;

#[derive(Component)]
pub struct PlayerGoldUi;

#[derive(Component)]
pub struct TreeUi;