
use bevy::prelude::*;
use console::ConsoleView;
use console::ConsolePlugin;

use engine2d::Engine2DView;
use engine2d::Engine2DPlugin;

pub mod dialogue_view;

mod engine2d;
mod console;

pub struct GraphicsPlugin;


impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugins(Engine2D);
        app.add_plugins(ConsolePlugin);
    }
}


pub trait DisplayView {
    fn display_dialogue(&self, content : &String);
}

fn get_display_view() -> &'static dyn DisplayView {
    static DISPLAY_VIEW: ConsoleView = ConsoleView;
    //static DISPLAY_VIEW: Engine2DView = Engine2DView;

    &DISPLAY_VIEW
}