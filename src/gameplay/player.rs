use bevy::{prelude::{Query, Transform, Camera, Res, Input, KeyCode, With, Vec3}, input::keyboard, time::Time};

const SPEED: f32 = 10.0;

pub fn player_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut pos in query.iter_mut() {
        if keys.pressed(KeyCode::Z) { // Walk
            // Get forward direction (negative z in camera space)
            let forward = pos.rotation.mul_vec3(Vec3::new(0.0, 0.0, -1.0));
            pos.translation += forward * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) { // Walk backwards
            let forward = pos.rotation.mul_vec3(Vec3::new(0.0, 0.0, -1.0));
            pos.translation -= forward * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::D) { // Go right
            let right = pos.rotation.mul_vec3(Vec3::new(-1.0, 0.0, 0.0));
            pos.translation -= right * SPEED * time.delta_seconds();
        }
        if keys.pressed(KeyCode::Q) { // Go left
            let right = pos.rotation.mul_vec3(Vec3::new(-1.0, 0.0, 0.0));
            pos.translation += right * SPEED * time.delta_seconds();
        }
    }
}