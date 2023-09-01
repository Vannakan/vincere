use bevy::prelude::*;
use crate::player::*;
use crate::common::components::BoundingBox;
use crate::player::components::Player;


#[derive(Component)]
pub struct Coin {
    pub amount: i32
}

#[derive(Component)]
pub struct Inventory {
    pub coins: i32,
}


#[derive(Bundle)]
pub struct GoldCoin {
    pub bounding_box: BoundingBox,
    pub coin: Coin,
    pub sprite: SpriteBundle,
} 

// #[derive(Event)]
// pub struct Collision<A: Component, B:Component>{
//     pub a: A,
//     pub b: B
// }

// fn intersect()

fn gizmos(mut gizmo: Gizmos, gold_q: Query<(Entity, &BoundingBox, &Transform)>){
    for gold in gold_q.iter(){
        gizmo.rect_2d(
            Vec2::from([gold.2.translation.x, gold.2.translation.y]),
            0.,
            Vec2::from([gold.1.width, gold.1.height]),
            Color::BLACK,
        );
}
}

pub fn player_gold_collision(mut commands: Commands, mut player_q: Query<(Entity, &Player, &BoundingBox, &Transform, &mut Inventory)>, gold_q: Query<(Entity, &Coin, &BoundingBox, &Transform)>, asset_server: Res<AssetServer>){

    if(player_q.is_empty() || gold_q.is_empty()) { return; }

    let (_,_, a_aabb, a_trans, mut inventory) = player_q.single_mut();
    for (b_entity, coin, b_aabb, b_trans) in gold_q.iter(){

        let a_pos = a_trans.translation;
        let b_pos = b_trans.translation;

        if (a_pos.x < b_pos.x + b_aabb.width && a_pos.x + a_aabb.width > b_pos.x && a_pos.y < b_pos.y + b_aabb.height && a_pos.y + a_aabb.height > b_pos.y)
            {
                println!("COLLIDING");
                inventory.coins += coin.amount;
                commands.get_entity(b_entity).unwrap().despawn();
                commands.spawn(AudioBundle{
                    source: asset_server.load("audio\\pickup_coin.wav"),
                    settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn, ..Default::default() },
                });
            }
    }
}

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, add_coins)
        .add_systems(Update, (player_gold_collision, gizmos));
    }
}

pub fn add_coins(mut commands: Commands, asset_server: Res<AssetServer>) {
       commands.spawn((GoldCoin{
        bounding_box: BoundingBox { width: 100.0, height: 100.0 },
        coin: Coin {
            amount: 10
        },
        sprite: SpriteBundle { transform: Transform { translation: Vec3::from([200.0, 10.0, 1.0]), ..default() }
            , texture: asset_server.load("gold.png"), ..Default::default()}
    }));
}

// pub fn gold_picked_up(mut reader: EventReader<CoinPickedUp>, mut query: Query<&mut Inventory>, mut commands: Commands, asset_server: Res<AssetServer>){
//     for evt in reader.iter() {
//         let mut inventory = match query.get_mut(evt.entity) {
//             Ok(e) => e,
//             _ => continue
//         };
//         commands.spawn(AudioBundle{
//             source: asset_server.load("/audio/gold_pickup.wav"),
//             settings: PlaybackSettings { mode: bevy::audio::PlaybackMode::Despawn, ..Default::default() },
//         });
//         inventory.coins += evt.amount;
//     }
// }