use bevy::prelude::*; //let you import bevy API




fn main() {
    App::new()
    .add_plugins((DefaultPlugins, MemoPlugin))
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


// GAME -- BOOK OF MEMENTO

pub struct MemoPlugin;

impl Plugin for MemoPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup);
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(SpriteBundle{
        texture: asset_server.load("sprites/traveler_sketch.png"),
        ..default()
    });
}