use bevy::prelude::*;

#[derive(Event)]
pub struct PushBack{
    pub from: Transform,
    pub to: Entity,
}


#[derive(Event)]
pub struct AttackEvent
{
  pub from:Transform,
  pub to: Entity,
  pub damage: f32   
}