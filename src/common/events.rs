use bevy::prelude::{Event, Component};

#[derive(Event)]
pub struct Collision<A: Component, B:Component>{
    pub a: A,
    pub b: B
}