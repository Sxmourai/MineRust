use bevy::{prelude::*, app::Startup, DefaultPlugins, window::{Window, WindowPosition}};
pub mod setup;
pub mod gameplay;
pub mod world;
pub mod camera;
pub mod bloc;

use bevy_rapier3d::{prelude::{NoUserData, RapierConfiguration}, render::{RapierDebugRenderPlugin, DebugRenderMode}};
use gameplay::player::*;
use setup::setup;
use world::{World, optimise_world, gen_world};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "MineRust".into(),
            resolution: (900., 1080.).into(),
            position: WindowPosition::At(IVec2::new(0, 0)),

            ..default()
        }),
        ..default()
    }),)
//     .add_plugins(bevy_editor_pls::prelude::EditorPlugin::default())
    .add_plugins(bevy_rapier3d::prelude::RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins((
        // it is fast unless debug rendering it on, it is too many lines so im only going to show the
        // AABB not the acctual shape for now
        RapierDebugRenderPlugin {
            mode: DebugRenderMode::empty(),
            ..Default::default()
        },
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        // bevy_diagnostics_explorer::DiagnosticExplorerAgentPlugin,
    ))
    .insert_resource(RapierConfiguration {
    gravity: Vec3::ZERO,
    ..default()
})
    // .add_plugin(bevy_inspector_egui_rapier::InspectableRapierPlugin)
    // .add_plugins(WorldInspectorPlugin::default()) // Laggy
    .insert_resource(RapierConfiguration {
        gravity: Vec3::new(0.0, 0., 0.0),
        ..default()
    })
    .add_systems(Startup, setup)
    .add_systems(Update, cursor_grab_system)
    .add_systems(Update, player_movement)
    // .add_systems(Startup, generate_world.after(setup)) // Reput .after (playermovement)
    .add_systems(Update,gen_world.after(player_movement))
    .add_systems(Update,player_on_ground.before(player_movement))
    .insert_resource(World::new(1))
    .insert_resource(JumpTimer(Timer::from_seconds(0.3, TimerMode::Once)))
    .run();
}