use bevy::prelude::*;
//use self::graphics::Graphics;
use self::logic::Logic;

mod graphics;
mod logic;

// PLUGIN
pub struct GuidesChapter;

impl Plugin for GuidesChapter {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(Logic);
        //.add_plugins(Graphics);
    }
}
