
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Velocity, Collider, Restitution, LockedAxes, Friction, Ccd, MassProperties, ExternalForce, Damping, ExternalImpulse, KinematicCharacterController, RigidBody, CollisionGroups, Group};
use noise::NoiseFn;

use crate::{world::{World, generate_world, BlocPosition}, gameplay::player::{PlayerVelocity, Player}, camera::CameraTag};


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
    let cy = world.get_height_at(cx, cz)+2.;
    let player_height = 1.75;
    let camera_bundle = Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::new(20., 5., 20.), Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection { fov: 89., ..default()}),
        ..default()
    };
    let camera = &camera_bundle.camera.clone();
    commands.spawn(camera_bundle).insert(CameraTag);
    println!("Spawning player ({}, {}, {})", cx,cy,cz);
    let transform = Transform::from_xyz(cx as f32, cy as f32, cz as f32);
    // ----------------------- Creating player
    let _player_id = commands
    .spawn(TransformBundle::from_transform(transform))
    .insert(Collider::cuboid(0.3, player_height/2., 0.2))
    // TODO Collision groups
    // .insert(KinematicCharacterController {
    //     offset: bevy_rapier3d::prelude::CharacterLength::Absolute(1.0),
    //     ..default()
    // })
    .insert(RigidBody::Dynamic)
    .insert(Friction::coefficient(0.0))
    .insert(Restitution::coefficient(0.))
    .insert(Damping {
        linear_damping: 0.0,
        ..default()
    })
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Player::new())
    .insert(Ccd::enabled())
    .insert(ExternalImpulse {
        impulse: Vec3::Y * 0.,
        ..default()
    })
    .insert(ExternalForce {
        force: Vec3::ZERO,
        ..default()
    })
    .insert(Velocity::default())
    .id()
    ;
    world.setup(commands, meshes, asset_server, materials, &transform, camera);
}