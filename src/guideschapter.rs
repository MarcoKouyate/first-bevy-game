use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*, render::render_resource::encase::rts_array::Length,
};

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



fn create_ui_list(title: &str, actions: Vec<String>, commands: &mut Commands, asset_server: &Res<AssetServer>){
    let height : f32  = 50.0 +  (40.0 * actions.length() as f32 );
    
    let root_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let left_vertical_fill_border = NodeBundle {
        style: Style {
            width: Val::Px(200.),
            height: Val::Px(height),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        background_color: Color::rgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let left_vertical_fill_content = NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            width: Val::Percent(100.),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    };

    let text_ui = TextBundle::from_section(
        title,
        TextStyle {
            font: asset_server.load("fonts/ShadowsIntoLight-Regular.ttf"),
            font_size: 30.0,
            ..default()
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(5.)),
        ..default()
    });

    // root node
    commands.spawn(root_node).with_children(|parent| {
        // left vertical fill (border)
        parent
            .spawn(left_vertical_fill_border)
            .with_children(|parent| {
                // left vertical fill (content)
                parent
                    .spawn(left_vertical_fill_content)
                    .with_children(|parent| {
                        // text
                        parent.spawn((
                            text_ui,
                            Label,
                        ));

                        parent.spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        }).with_children(|parent| {
                            // List items
                            for action in actions {
                                parent.spawn(list_item_ui(asset_server, &action));
                            }
                        });
                    });
            });
    });
}


// LIST ITEM
fn list_item_ui(
    asset_server: &Res<AssetServer>,
    action: &String
) -> (TextBundle, Label, AccessibilityNode) {
    (
        TextBundle::from_section(
            action,
            TextStyle {
                font: asset_server.load("fonts/ShadowsIntoLight-Regular.ttf"),
                font_size: 40.,
                ..default()
            },
        ),
        Label,
        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
    )
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
    create_ui_list("Example",  actions, commands, asset_server);
}
