use bevy::prelude::*;

#[derive(Component)]
pub struct Builder{
    pub trees: i32,
    pub last_action: f32,
}

#[derive(Component)]
pub struct Target(pub Entity);

#[derive(Component)]
pub struct FindTree;

#[derive(Component)]
pub struct MoveToTree {
    pub entity: Entity
}

#[derive(Component)]
pub struct CutTree;

#[derive(Component)]
pub struct MoveToCampsite;

#[derive(Component)]
pub struct DepositCampsite;

pub struct BuilderPlugin;