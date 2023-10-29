use bevy::{prelude::{App, Update}, app::Startup, DefaultPlugins};
mod setup;
mod gameplay;
use gameplay::player::*;
use setup::setup;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, player_movement)
    .add_systems(Update, cursor_grab_system)
    // .insert_resource(WindowDescriptor {
    //     title: "MineRust".to_string(),
    //     width: 640.0,
    //     height: 400.0,
    //     vsync: true,
    //     ..Default::default()
    // })
    .run();
}