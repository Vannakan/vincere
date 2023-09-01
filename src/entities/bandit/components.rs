use bevy::prelude::*;

#[derive(Component)]
pub struct Bandit;

#[derive(Component)]
pub struct Idle;


#[derive(Component)]
pub struct MoveToMinion(pub Entity, pub Transform);

#[derive(Component)]
pub struct AttackMinion(pub Entity);