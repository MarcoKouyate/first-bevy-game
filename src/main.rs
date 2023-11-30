use bevy::prelude::*; //let you import bevy API

fn main() {
    App::new()
    .add_systems(Update, hello_world)
    .run();
}

fn hello_world(){
    println!("Hello World! This is my first game with Bevy");
}