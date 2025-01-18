
use bevy::prelude::*;
use crate::tools::Queue;
use crate::guideschapter::graphics::dialogue_view::display_dialogue;

pub struct DialogueSystem;

impl Plugin for DialogueSystem {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, create_dialogue_system)
        .add_systems(Update, create_dialogues)
        // .add_systems(Update, retrieve_next_dialogue_as_current)
        // .add_systems(Update, press_space_to_clear_dialogue);
        .add_systems(Update, add_next_dialogue_on_press)
        .add_systems(Update, display_current_line);
    }
}

#[derive(Resource)]
struct DialogueResource {
    dialogues: Queue<Entity>,
    current : Option<Entity>
}

#[derive(Component)]
struct Dialogue(String);

fn create_dialogue_system(mut commands: Commands) {
    let dialogue_system = DialogueResource {
        dialogues: Queue::new(),
        current: None
    };

    commands.insert_resource(dialogue_system);
}


fn create_dialogues(mut dialog_system : ResMut<DialogueResource>, mut commands : Commands){
    create_dialogue(&mut dialog_system, &mut commands, String::from("Test de dialogue"));
    create_dialogue(&mut dialog_system, &mut commands, String::from("Et hop!"));
}

fn create_dialogue(dialog_system : &mut ResMut<DialogueResource>, commands : &mut Commands, content : String){
    let dialogue = commands.spawn(Dialogue(content));
    dialog_system.dialogues.enqueue(dialogue.id());
}


fn display_current_line(
    dialogue_system: ResMut<DialogueResource>,
    commands : Commands,
    query : Query<&Dialogue>
){

    match dialogue_system.current {
        Some(current) => {

            if let Ok(dialogue) = query.get(current){
                display_dialogue(&dialogue.0);
            }
            clear_current_line(current, dialogue_system,  commands);
        }
        None => {
            
        }
    }
}

fn add_next_dialogue_on_press(
    keyboard_input: Res<Input<KeyCode>>,
    mut dialogue_system: ResMut<DialogueResource>
){
    if keyboard_input.just_pressed(KeyCode::Space) {
        dialogue_system.current = dialogue_system.dialogues.dequeue();
    }
}


fn clear_current_line(line : Entity, mut  dialogue_system: ResMut<DialogueResource>, mut commands : Commands){
    dialogue_system.current = Option::None;
    commands.entity(line).despawn();
}
