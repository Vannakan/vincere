use bevy::prelude::*;

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Elaina".to_string())));
    commands.spawn((Person, Name("Markus".to_string())));
    commands.spawn((Person, Name("Simon".to_string())));
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("Hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0)
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people));
    }
}