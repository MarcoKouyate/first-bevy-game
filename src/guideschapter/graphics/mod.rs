
use bevy::prelude::*;
use self::{inspector::CustomInspectorPlugin, selection::SelectionPlugin};

mod selection_menu;
mod selection;
mod inspector;

pub struct Graphics;

impl Plugin for Graphics {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(CustomInspectorPlugin)
        .add_plugins(SelectionPlugin)
        .add_systems(Startup, setup);
    }
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


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_ui(&mut commands, &asset_server);
}
