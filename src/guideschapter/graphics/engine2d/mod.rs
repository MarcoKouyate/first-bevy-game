
use bevy::prelude::*;
use self::user_interface::UserInterfacePlugin;
use crate::guideschapter::graphics::DisplayView;

mod user_interface;


pub struct Engine2DPlugin;

impl Plugin for Engine2DPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(UserInterfacePlugin);
    }
}

pub struct Engine2DView;

impl DisplayView for Engine2DView {
    fn display_dialogue(&self, content: &String){
        // TODO: handle dialogue for Engine2D
        println!("ENGINE2D: {}", content);
    }
}

