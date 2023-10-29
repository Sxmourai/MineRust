use bevy::{prelude::App, app::Startup, DefaultPlugins};
mod setup;
use setup::setup;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}