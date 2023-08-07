use bevy::prelude::*;

use crate::Velocity;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

pub fn movement(
    mut movement_query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>,){
    for movement in movement_query.iter_mut(){
        let (mut transform, mut velocity) = movement;

        transform.translation += velocity.0 * time.delta_seconds();
    
        if velocity.0.x >= -0.1 && velocity.0.x <= 0.1 && velocity.0.y <= 0.1 && velocity.0.y >= -0.1
        {
            velocity.0 = Vec3::default();
        }
        else {
            velocity.0 = velocity.0.lerp(Vec3::default(), 0.1)
        }
    }
}

// pub fn avoidance(mut bandit_query: Query<(&mut Transform, &mut Velocity), With<Bandit>>){
//     let mut combinations = bandit_query.iter_combinations_mut();
//     while let Some([mut t1, t2]) = combinations.fetch_next(){
//         if t1.0.translation.distance(t2.0.translation) > 50.0 { continue;}
//         let t1_pos = t1.0.translation;
//         let t2_pos = t2.0.translation;
//         t1.borrow_mut().1.0 += avoidance(t1_pos,t2_pos)
//     }
// }
