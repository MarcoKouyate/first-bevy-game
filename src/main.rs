use bevy::prelude::*; //let you import bevy API

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, HelloPlugin))
    .run();
}

// PLUGINS
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, (hello_world, add_people))
        .add_systems(Update, greet_people);
    }
}


// COMPONENTS
#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);



// SYSTEMS
fn hello_world(){
    println!("Hello World! This is my first game with Bevy");
}

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Memento".to_string())));
    commands.spawn((Person, Name("Aqualya".to_string())));
    commands.spawn((Person, Name("Revoli".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query{
        println!("Hello, {}!", name.0);
    }
}