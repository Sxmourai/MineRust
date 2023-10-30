
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Velocity, Collider, Restitution, LockedAxes, Friction, Ccd};
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 10.),
        ..default()
    });
    // camera
    let (cx,cy,cz) = (0.,20.,0.);
    let player_height = 1.75; 
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(cx,cy+player_height,cz).looking_at(Vec3::new(20., 5., 20.), Vec3::Y),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, player_height, 1.0))),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(cx,cy,cz),
        ..default()
    })
    .insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1., 2., 1.) )),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-5., 50., -5.),
        ..default()
    })
    .insert(bevy_rapier3d::prelude::RigidBody::Dynamic)            
    .insert(Velocity {
        linvel: Vec3::new(0.0, 0.0, 0.0),
        angvel: Vec3::new(0.0, 0.0, 0.0),
    })
    .insert(Friction::coefficient(0.7))
    .insert(Restitution::coefficient(0.))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Collider::cuboid(0.7, 1.4, 0.7)); // Player dimensions


}