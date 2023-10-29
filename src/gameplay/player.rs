use std::sync::Mutex;

use bevy::{prelude::*, input::{keyboard, mouse::MouseMotion}, time::Time, window::{CursorMoved, CursorGrabMode, WindowFocused, PrimaryWindow}};

const SPEED: f32 = 10.0;
const SENSITIVITY: f32 = 0.00005;

pub fn player_movement(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut camera = query.single_mut();
    let forward = camera.rotation.mul_vec3(Vec3::new(0.0, 0.0, -1.0));
    let right = camera.rotation.mul_vec3(Vec3::new(-1.0, 0.0, 0.0));
    if keys.pressed(KeyCode::Z) { // Walk
        // Get forward direction (negative z in camera space)
        camera.translation += forward * SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) { // Walk backwards
        camera.translation -= forward * SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D) { // Go right
        camera.translation -= right * SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Q) { // Go left
        camera.translation += right * SPEED * time.delta_seconds();
    }
    // mut state: ResMut<InputState>,
    let window = primary_window.single();
    for ev in motion_evr.iter() {
        let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
        match window.cursor.grab_mode {
            CursorGrabMode::None => (),
            _ => {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                pitch -= (SENSITIVITY * ev.delta.y * window_scale).to_radians();
                yaw -= (SENSITIVITY * ev.delta.x * window_scale).to_radians();
            }
        }
        pitch = pitch.clamp(-1.54, 1.54);
        // Order is important to prevent unintended roll
        camera.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}
//     let window = windows.single_mut();
//     if window.cursor.visible == false {
//         for ev in motion_evr.iter() {
//             camera.rotation = Quat::from_rotation_y(-ev.delta.x * SENSITIVITY)
//             * Quat::from_rotation_x(-ev.delta.y * SENSITIVITY)
//             * camera.rotation;
//         }
//    }

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