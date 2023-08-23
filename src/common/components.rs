use bevy::prelude::{Component, Vec3, Entity};

// // Movement
#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct BoundingBox {
    pub width: f32,
    pub height: f32
}

#[derive(Component)]
pub struct FollowPlayer;

#[derive(Component)]
pub struct Target{
    pub entity: Entity
}

#[derive(Component)]
pub struct Targetable;

// #[derive(Component)]
// pub struct Target(pub Entity);

// // Collision
// #[derive(Event)]
// pub struct Collision<A: Component, B:Component>{
//     pub a: A,
//     pub b: B
// }

// #[derive(Component)]
// pub struct BoundingBox {
//     pub width: f32,
//     pub height: f32
// }

// // Entity
// #[derive(Component)]
// pub struct Inventory {
//     pub coins: i32,
// }

// #[derive(Component)]
// pub struct Prop;
