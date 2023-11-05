use bevy::{prelude::*, app::Startup, DefaultPlugins, window::{Window, WindowPosition}, log::LogPlugin};
pub mod setup;
pub mod gameplay;
pub mod world;
pub mod camera;
pub mod bloc;

use bevy_rapier3d::{prelude::{NoUserData, RapierConfiguration}, render::{RapierDebugRenderPlugin, DebugRenderMode}};
use gameplay::{player::*, mobs::{SpawnTimer, spawn_animals, animal_live}};
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
    }).disable::<LogPlugin>(),)
    
//     .add_plugins(bevy_editor_pls::prelude::EditorPlugin::default())
    .add_plugins(bevy_rapier3d::prelude::RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins((
        RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..Default::default()
        },
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy_diagnostics_explorer::DiagnosticExplorerAgentPlugin,
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
    .add_systems(Update,gen_world.after(player_movement))
    .add_systems(Update, optimise_world.after(gen_world))
    .add_systems(Update,player_on_ground.before(player_movement))
    .add_systems(Update,text_update.after(player_movement))
    .add_systems(Update,animal_live)
    .add_systems(Update,spawn_animals)
    .insert_resource(World::new(1))
    .insert_resource(JumpTimer(Timer::from_seconds(0.3, TimerMode::Once)))
    .insert_resource(SpawnTimer(Timer::from_seconds(3., TimerMode::Once)))
    .run();
}