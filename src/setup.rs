
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Velocity, Collider, Restitution, LockedAxes, Friction, Ccd, MassProperties, ExternalForce, Damping, ExternalImpulse, KinematicCharacterController, RigidBody, CollisionGroups, Group};
use noise::NoiseFn;

use crate::{world::{World, generate_world, BlocPosition}, gameplay::player::{PlayerVelocity, Player, spawn_player}, camera::CameraTag};


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<World>,
) {
    // Sun
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 20., 0.),
        ..default()
    });

    
    let (cx,cz) = (5.,5.);
    let cy = world.get_height_at(cx, cz);
    println!("Spawning player ({}, {}, {})", cx,cy,cz);
    let transform = Transform::from_xyz(cx as f32, cy as f32, cz as f32);
    spawn_player(&mut commands, transform);
    world.setup(commands, meshes, asset_server, materials, &transform);
}