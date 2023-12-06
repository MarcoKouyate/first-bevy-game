use bevy::prelude::*;

mod selection_menu;

// PLUGIN
pub struct MemoPlugin;

impl Plugin for MemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

// SETUP
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_scene(&mut commands, &asset_server);
    setup_ui(&mut commands, &asset_server);
}

fn setup_scene(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/fire.png"),
        ..default()
    });
}


fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {

    let actions = vec![
        String::from("Test 1"),
        String::from("Test 2"),
        String::from("Test 1"),
        String::from("Test 2"),
        String::from("Test 1"),
        String::from("Test 2"),
        String::from("Test 3")
    ];
    selection_menu::create_ui_list("Example",  actions, commands, asset_server);
}
