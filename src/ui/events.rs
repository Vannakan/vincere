use bevy::prelude::{Event, Entity};

#[derive(Event)]
pub struct DestroyUi(pub Entity);

#[derive(Event)]
pub struct BindUi(pub Entity, pub String);

#[derive(Event)]
pub struct EntityAttacked{
    pub entity: Entity,
    pub health_left: f32,
    pub starting_health: f32
}