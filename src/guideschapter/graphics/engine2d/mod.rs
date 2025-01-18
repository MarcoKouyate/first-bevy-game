
use bevy::prelude::*;
use self::user_interface::UserInterfacePlugin;
use crate::guideschapter::graphics::DialogueView;

mod user_interface;


pub struct Engine2D;

impl Plugin for Engine2D {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(UserInterfacePlugin);
    }
}

impl DialogueView for Engine2D {
    fn display_dialogue(&self, content: &String){
        // TODO: handle dialogue for Engine2D
    }
}

