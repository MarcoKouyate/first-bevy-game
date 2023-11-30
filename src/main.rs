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
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, add_people)
        .add_systems(Update, greet_people);
    }
}


// COMPONENTS
#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);


// RESOURCES
#[derive(Resource)]
struct GreetTimer(Timer);


// SYSTEMS
fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Memento".to_string())));
    commands.spawn((Person, Name("Aqualya".to_string())));
    commands.spawn((Person, Name("Revoli".to_string())));
}


fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}