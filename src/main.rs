use bevy::prelude::*;

// GUIDE'S CHAPTER - A visual novel
mod guideschapter;
use guideschapter::GuidesChapter;

// Test module
mod greetings;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, GuidesChapter))
    .run();
}