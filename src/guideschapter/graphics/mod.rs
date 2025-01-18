
use bevy::prelude::*;
//use self::engine2d::Engine2D;
use self::console::ConsolePlugin;

mod engine2d;
mod console;

pub struct GraphicsPlugin;

pub trait DialogueView {
    fn display_dialogue(&self, content : &String);
}


impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugins(Engine2D);
        app.add_plugins(ConsolePlugin);
    }
}






