
use bevy::prelude::*;
use crate::guideschapter::graphics::DialogueView;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {

    }
}

impl DialogueView for ConsolePlugin {
    fn display_dialogue(&self, content: &String){
        print!("{}", content);
    }
}