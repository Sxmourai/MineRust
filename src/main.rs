use bevy::{prelude::*, app::Startup, DefaultPlugins, asset::ChangeWatcher, window::{Window, WindowPosition}, utils::HashMap};
mod setup;
mod gameplay;
mod world;
use gameplay::player::*;
use setup::setup;
use world::{generate_world, WorldMap};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "MineRust".into(),
            resolution: (900., 1080.).into(),
            position: WindowPosition::At(IVec2::new(1000, 0)),

            ..default()
        }),
        ..default()
    }),)
    .add_systems(Startup, setup)
    .add_systems(Startup, generate_world)
    .add_systems(Update, player_movement)
    .add_systems(Update, cursor_grab_system)
    .insert_resource(WorldMap(HashMap::new()))
    // .insert_resource(WindowDescriptor {
    //     title: "MineRust".to_string(),
    //     width: 640.0,
    //     height: 400.0,
    //     vsync: true,
    //     ..Default::default()
    // })
    .run();
}