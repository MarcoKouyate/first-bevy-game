

use bevy::prelude::*;

pub struct Logic;

impl Plugin for Logic {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, create_characters)
        .add_systems(Startup, create_dialogue_system)
        .add_systems(Update, create_dialogues)
        .add_systems(Update, read_dialogue);
    }
}


#[derive(Resource)]
struct DialogueSystem {
    dialogues: Queue<Entity>
}



#[derive(Component)]
struct Dialogue(String);


#[derive(Component)]
struct Character;

#[derive(Component)]
struct Name(String);





fn create_characters(mut commands : Commands){
    commands.spawn((Character, Name(String::from("Traveler"))));
    //commands.spawn(Dialogue);
}


fn create_dialogue_system(mut commands: Commands) {
    let dialogue_system = DialogueSystem {
        dialogues: Queue::new()
    };

    commands.insert_resource(dialogue_system);
}

//QUEUE
struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    // Constructor for creating a new empty queue
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    // Method to add an element to the back of the queue
    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    // Method to remove and return the front element of the queue
    fn dequeue(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    // Method to check if the queue is empty
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    // Method to view the front element without removing it
    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}



fn create_dialogues(mut dialog_system : ResMut<DialogueSystem>, mut commands : Commands){
    create_dialogue(&mut dialog_system, &mut commands, String::from("Test de dialogue"));
    create_dialogue(&mut dialog_system, &mut commands, String::from("Et hop!"));
}

fn create_dialogue(dialog_system : &mut ResMut<DialogueSystem>, commands : &mut Commands, content : String){
    let dialogue = commands.spawn(Dialogue(content));
    dialog_system.dialogues.enqueue(dialogue.id());
}

fn read_dialogue(
    keyboard_input: Res<Input<KeyCode>>,
    mut dialogue_system: ResMut<DialogueSystem>,
    query : Query<&Dialogue>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("SPACE");
        let dialogue_entity = dialogue_system.dialogues.dequeue().unwrap();

        if let Ok(dialogue) = query.get(dialogue_entity){
            println!("{}", dialogue.0);
        }
    }
}
