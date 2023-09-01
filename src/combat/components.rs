use std::marker::PhantomData;

use bevy::prelude::{Event, Component, Entity, Transform};


#[derive(Component)]
pub struct Attackable;

#[derive(Component)]
pub struct FindTarget<T: Component>{
    pub phantom: PhantomData<T>
}


#[derive(Component)]
pub struct AttackInfo{
    pub range: f32,
    pub cooldown: f32,
    pub last_attacked: f32
}

#[derive(Event)]
pub struct Attack2<'a>
{
  pub from: &'a Transform,
  pub to: Entity,
  pub damage: f32,
}

#[derive(Component)]
pub struct Attacks {
    pub last_attacked: f32
}

#[derive(Component)]
pub struct Health{
    pub starting: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct FoundTarget<T: Component>{
    pub me: Entity,
    pub to: Entity,
    phantom: PhantomData<T>
}

#[derive(Component)]
pub struct HasTarget{
    pub target: Option<Entity>,
}
