use bevy::prelude::*;
use crate::{world::World, gameplay::player::spawn_player};

pub fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<World>,
) {
    // Sun
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10., 10., 10.),
        ..default()
    });

    
    let (cx,cz) = (5.,5.);
    let cy = world.get_height_at(cx, cz);
    println!("Spawning player ({}, {}, {})", cx,cy,cz);
    let transform = Transform::from_xyz(cx as f32, cy as f32, cz as f32);
    spawn_player(&mut commands, transform);

    world.setup(commands, meshes, asset_server, materials, &transform);
}