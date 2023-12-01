use bevy::prelude::*; //let you import bevy API




fn main() {
    App::new()
    .add_plugins((DefaultPlugins, MemoPlugin))
    .run();
}

// PLUGINS
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App){
        app
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, add_people)
        .add_systems(Update, greet_people);
    }
}


// COMPONENTS
#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);


// RESOURCES
#[derive(Resource)]
struct GreetTimer(Timer);


// SYSTEMS
fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Memento".to_string())));
    commands.spawn((Person, Name("Aqualya".to_string())));
    commands.spawn((Person, Name("Revoli".to_string())));
}


fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}


// GAME -- BOOK OF MEMENTO

pub struct MemoPlugin;

impl Plugin for MemoPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup);
    }
}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    setup_scene(&mut commands, &asset_server);
    setup_ui(&mut commands, &asset_server);
}

fn setup_scene(commands: &mut Commands, asset_server: &Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(SpriteBundle{
        texture: asset_server.load("sprites/fire.png"),
        ..default()
    });
}

fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>){
    let root_node =    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };
    
    let  left_vertical_fill_border =     NodeBundle {
        style: Style {
            width: Val::Px(200.),
            height: Val::Px(500.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        background_color: Color::rgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let left_vertical_fill_content = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    };

    let text_ui = TextBundle::from_section(
        "Text Example",
        TextStyle {
            font: asset_server.load("fonts/ShadowsIntoLight-Regular.ttf"),
            font_size: 30.0,
            ..default()
        },
    ).with_style(Style {
        margin: UiRect::all(Val::Px(5.)),
        ..default()
    });


    // root node
    commands
        .spawn(root_node).
        with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(left_vertical_fill_border)
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(left_vertical_fill_content)
                        .with_children(|parent| {
                            // text
                            parent.spawn(    (
                                text_ui
                                ,
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });
    });
}