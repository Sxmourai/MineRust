use std::sync::Mutex;

use bevy::{prelude::*, input::{keyboard, mouse::MouseMotion}, time::Time, window::{CursorMoved, CursorGrabMode, WindowFocused, PrimaryWindow}};

const SPEED: f32 = 10.0;
const SENSITIVITY: f32 = 0.002;

pub fn player_movement(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
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
        let window = windows.single_mut();
        if window.cursor.visible == false {
            for ev in motion_evr.iter() {
                pos.rotation = Quat::from_rotation_y(-ev.delta.x * SENSITIVITY)
                * Quat::from_rotation_x(-ev.delta.y * SENSITIVITY)
                * pos.rotation;

            }
        }
    }
}
pub fn cursor_grab_system(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}