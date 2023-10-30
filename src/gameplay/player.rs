use std::sync::Mutex;

use bevy::{prelude::*, input::{keyboard, mouse::MouseMotion}, time::Time, window::{CursorMoved, CursorGrabMode, WindowFocused, PrimaryWindow}};
use bevy_rapier3d::prelude::{Velocity, RigidBody, Collider};

const SPEED: f32 = 10.0;
const SENSITIVITY: f32 = 0.00005;
const GRAVITY: f32 = 1.;

pub fn player_movement(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Velocity, &mut Transform, &RigidBody), With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let (mut vel, mut camera, _body) = query.single_mut();
    camera.translation += vel.linvel;
    let mut forward = camera.rotation.mul_vec3(Vec3::NEG_Z);
    let mut right = camera.rotation.mul_vec3(Vec3::new(-1.0, 0.0, 0.0));
    forward.y = 0.;right.y = 0.;
    let mut vector_direction = Vec3::ZERO;
    if keys.pressed(KeyCode::Z) { // Walk
        vector_direction += forward;
    }
    if keys.pressed(KeyCode::S) { // Walk backwards
        vector_direction -= forward;
    }
    if keys.pressed(KeyCode::D) { // Go right
        vector_direction -= right;
    }
    if keys.pressed(KeyCode::Q) { // Go left
        vector_direction += right;
    }
    if keys.pressed(KeyCode::Space) { // Go up
        vector_direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) { // Go down
        vector_direction += Vec3::NEG_Y;
    }
    vel.linvel = vector_direction * SPEED * time.delta_seconds();
    vel.linvel += Vec3::NEG_Y * GRAVITY * time.delta_seconds();
    // vel.linvel = (vel.linvel + vector_direction).min(Vec3 { x: 3., y: 3., z: 3. });
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