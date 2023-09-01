use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component)]
pub struct Builder{
    pub trees: i32,
  
}
#[derive(Component)]
pub struct Action {
    pub last_action: f32,
}

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

#[derive(Component)]
pub struct MovingTo<T: Component>{
    pub phantom: PhantomData<T>
}
