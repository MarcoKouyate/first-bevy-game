use bevy::prelude::*;
use self::graphics::Graphics;

mod graphics;

// PLUGIN
pub struct GuidesChapter;

impl Plugin for GuidesChapter {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(Graphics)
        .add_systems(Startup, setup);
    }
}

// SETUP
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_scene(&mut commands, &asset_server);
}

fn setup_scene(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/fire.png"),
        ..default()
    });
}
