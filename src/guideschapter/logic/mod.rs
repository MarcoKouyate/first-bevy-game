

use bevy::prelude::*;
use self::dialogue_system::DialogueSystem;

mod dialogue_system;

pub struct Logic;

impl Plugin for Logic {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(DialogueSystem)
        .add_systems(Startup, create_characters);
    }
}


#[derive(Component)]
struct Character;

#[derive(Component)]
struct Name(String);





fn create_characters(mut commands : Commands){
    commands.spawn((Character, Name(String::from("Traveler"))));
    //commands.spawn(Dialogue);
}


