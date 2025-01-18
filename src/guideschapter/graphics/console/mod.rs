
use bevy::prelude::*;
use crate::guideschapter::graphics::DisplayView;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {

    }
}

pub struct  ConsoleView;
impl DisplayView for ConsoleView {
    fn display_dialogue(&self, content: &String){
        println!("CONSOLE: {}", content);
    }
}