
use bevy::prelude::*;
pub fn setup(
    mut commands: Commands,
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.,20.,0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}