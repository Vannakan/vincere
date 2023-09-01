use bevy::prelude::*;

use crate::{game::{gold::Inventory, campsite::Campsite, camera::Minimap}, player::components::Player};

use super::{components::{UiTransformBinding, HasUi, EntityUiRoot, Root, HealthBar, TreeUi, PlayerGoldUi}, events::{DestroyUi, BindUi, EntityAttacked}, X_OFFSET, Y_OFFSET};
pub fn destroy_ui(
    mut commands: Commands,
    query_text: Query<(Entity, &EntityUiRoot, &UiTransformBinding), (Without<HasUi>, Without<Camera>)>,
    mut reader: EventReader<DestroyUi>)
{
    if query_text.is_empty() { return;}
    for entity in reader.iter()
    {
        let entity_to_destroy = entity.0;
        if let Some(ui) = query_text.iter().find(|t| t.2.ui_entity == entity_to_destroy)
        {
            commands.entity(ui.0).despawn_recursive();
        }
    }
}

pub fn entity_ui_movement (
    mut query_text: Query<(Entity,  &mut Style, &UiTransformBinding), (Without<HasUi>, Without<Camera>, Without<EntityUiRoot>)>, 
    mut query_minion: Query<(Entity, &mut Transform, &HasUi), (Without<Text>, Without<Camera>)>, 
    mut camera_query: Query<(&Camera, &GlobalTransform, &OrthographicProjection), (Without<HasUi>, Without<Text>, Without<Minimap>)>)
    {
        if query_minion.is_empty() { return ;}
        let camera = camera_query.single_mut();

        for mut ui in query_text.iter_mut()
        {
            if let Some(entity) = query_minion.iter_mut().find(|e| 
            {
                e.0 == ui.2.ui_entity
            })
            {
                if let Some(val) =  camera.0.world_to_viewport(camera.1, entity.1.translation) 
                {
                    ui.1.left =  Val::Px(val.x + X_OFFSET);
                    ui.1.top =  Val::Px(ui.2.y_offset + val.y + (Y_OFFSET /camera.2.scale)) ;
                }
            }
        }
}

pub fn create_ui_binding(mut event_reader: EventReader<BindUi>, query: Query<Entity, With<Root>>, asset_server: ResMut<AssetServer>, mut commands: Commands){
    let root = query.single();
    for evt in event_reader.iter() {
        // Fix this
        let binding = UiTransformBinding {
            ui_entity: evt.0,
            y_offset: 0.0
        };

        let binding2 = UiTransformBinding {
            ui_entity: evt.0,
            y_offset: -10.0,
        };

        let binding3 = UiTransformBinding {
            ui_entity: evt.0,
            y_offset: -10.0,
        };

        let binding4 = UiTransformBinding {
            ui_entity: evt.0,
            y_offset: -10.0,
        };
        add_ui(&mut commands, root, &asset_server, binding, binding2, binding3,binding4,&evt.1);
    }
}

pub fn on_attacked_ui_system(
    mut events: EventReader<EntityAttacked>,
    mut hp: Query<(&mut Style, &UiTransformBinding), With<HealthBar>>
    ){
    if events.is_empty() { return; }

    for evt in events.iter()
    {
        let e = hp.iter_mut().find(|x|x.1.ui_entity == evt.entity);

        let damage_percent = ((evt.starting_health - evt.health_left) / evt.starting_health) * 100.0;

        let health_bar_remaining = 30.0 - (30.0 / (100.0/damage_percent));
        if let Some(mut e) = e
        {
            e.0.width = Val::Px(health_bar_remaining)
        }
    }
}


pub fn add_ui(
    commands: &mut Commands,
    root: Entity, asset_server: &ResMut<AssetServer>,
    binding: impl Component,
    binding2: impl Component, 
    binding3: impl Component,  
    binding4: impl Component, 
    text: &String)
    {
    println!("ADD UI for {}", text);
    let entity_ui_root = commands
    .spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    },binding4,
    EntityUiRoot)).with_children(|parent| {

        // text
        parent.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("Kenney Pixel Square.ttf"),
                    font_size: 10.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.)),
                position_type: PositionType::Absolute,
                ..default()
            }),
            binding,
        ));

        //background hp
        parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(30.0),
                        height: Val::Px(5.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::RED.into(),
                    ..Default::default()
                }, binding2));

        // current hp
        parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(30.0),
                            height: Val::Px(5.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        background_color: Color::GREEN.into(),
                        ..Default::default()
                    }, binding3, HealthBar));

    }).id();
    
    commands.get_entity(root).unwrap().add_child(entity_ui_root);
}



pub fn create_tree_ui(mut commands: Commands, query: Query<(Entity, &Node, &Root)>, asset_server: Res<AssetServer>)
{
    let mut root = commands.get_entity(query.single().0).unwrap();

    root.with_children(|parent| {
        parent.spawn((
        TextBundle::from_section(
            "Trees:",
            TextStyle {
                font: asset_server.load("Kenney Pixel Square.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        }), TreeUi));
    });
}

pub fn update_tree_ui(mut query: Query<(Entity, &TreeUi, &mut Text)>, c_query: Query<&Campsite>)
{
    if query.is_empty() || c_query.is_empty() { return; }
    let mut text = query.single_mut();
    let campsite = c_query.single();
    text.2.sections[0].value = format!("TREES: {}", campsite.trees)
}


pub fn create_ui_root(mut commands: Commands)
{
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }, 
        Root));
}

pub fn create_gold_ui(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<(Entity, &Node, &Root)>){

    if(query.is_empty()) { return; }
    let mut root = commands.get_entity(query.single().0).unwrap();

    root.with_children(|parent| {
        parent.spawn(
        (TextBundle::from_section(
            "Gold:",
            TextStyle {
                font: asset_server.load("Kenney Pixel.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        ),
        PlayerGoldUi));
    });
}



pub fn update_player_gold_ui(mut p_query: Query<(&Player, &Inventory)>, mut ui_query: Query<(&PlayerGoldUi, &mut Text)>){
    if p_query.is_empty() || ui_query.is_empty() { return ;}
    let (_, inventory ) = p_query.single();
    let (_, mut text) = ui_query.single_mut();

    text.sections[0].value = format!("Gold: {}", inventory.coins);
}

pub fn create_player_ui(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&Player>){
    if query.is_empty() { return; }
    commands.spawn(ImageBundle{
        style: Style { 
            width: Val::Px(128.0),
            height: Val::Px(128.0),
            right: Val::Px(700.0),
            bottom: Val::Px(200.0),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        image: asset_server.load("war-horn.png").into(),
        ..Default::default()
    });
}


pub fn create_peasant_ui(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(ImageBundle{
        style: Style { 
            width: Val::Px(128.0),
            height: Val::Px(128.0),
            right: Val::Px(700.0),
            bottom: Val::Px(200.0),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        image: asset_server.load("war-horn.png").into(),
        ..Default::default()
    });
}