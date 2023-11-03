use std::sync::Mutex;

use bevy::{prelude::*, input::{keyboard, mouse::MouseMotion}, time::Time, window::{CursorMoved, CursorGrabMode, WindowFocused, PrimaryWindow}};
use bevy_rapier3d::{prelude::{Velocity, RigidBody, Collider, ExternalImpulse, ExternalForce, RapierConfiguration, KinematicCharacterController}, rapier::prelude::ColliderSet, na::ComplexField};
use crate::{setup::{Player, CameraTag}, world::{World, BlocPosition}};

#[derive(Resource)]
pub struct JumpTimer(pub Timer);

const SPEED: f32 = 5000.0;
const SENSITIVITY: f32 = 0.00005;

#[derive(Component, Default)]
pub struct PlayerVelocity {
    pub movement_vel: Vec3,
    pub forward_vel: Vec3,
    pub old_vel: Vec3,
    pub gravity: Vec3,
}

fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a * (1.0 - f) + (b * f)
}
fn vlerp(a: Vec3, b: Vec3, f: Vec3) -> Vec3 {
    a * (1.0 - f) + (b * f)
}

pub fn player_movement(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<CameraTag>>,
    mut player_query: Query<(&mut Velocity, &mut Player, &mut PlayerVelocity,&mut ExternalForce,&mut ExternalImpulse, &Transform), (With<Player>, Without<CameraTag>)>,
    // mut impulses: Query<&mut ExternalImpulse>,
    keys: Res<Input<KeyCode>>,
    time: ResMut<Time>,
    mut motion_evr: EventReader<MouseMotion>,
    mut timer: ResMut<JumpTimer>,
    physic_config: Res<RapierConfiguration>,
) {
    let mut camera = camera_query.single_mut();
    let (mut vel,mut player, mut player_vel, mut player_force, mut player_impulse, pos) = player_query.single_mut();
    let mut forward = camera.rotation.mul_vec3(Vec3::NEG_Z);
    let mut right = camera.rotation.mul_vec3(Vec3::NEG_X);
    forward.y = 0.;right.y = 0.;
    forward = forward.normalize();
    right = right.normalize();
    let mut vector_direction = Vec3::ZERO;
    player_vel.forward_vel = Vec3::ZERO;
    let mut p_vel = Vec3::ZERO;
    let speed = 10.1;
    let mut direction = Vec3::ZERO;
    
    player_vel.gravity += Vec3::NEG_Y * 0.5;
    if player.on_ground {
        player_vel.gravity = Vec3::ZERO;
    }
    if keys.pressed(KeyCode::Z) {
        direction += forward*speed;
    }
    if keys.pressed(KeyCode::S) {
        direction -= forward*speed;
    }
    if keys.pressed(KeyCode::Q) {
        direction += right*speed;
    }
    if keys.pressed(KeyCode::D) {
        direction -= right*speed;
    }
    timer.0.tick(time.delta());
    if keys.pressed(KeyCode::Space) && player.on_ground { // Jump
        if timer.0.finished() {
            player_vel.gravity = Vec3::Y*6.1; 
            println!("Jumped {}", player_vel.gravity);
            timer.0.reset();
        }
    }
    if keys.pressed(KeyCode::ShiftLeft) { // Sneak
    }
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    // Multiplie par la vitesse maximale souhaitée
    let max_speed = 5.0; // Remplacez 5.0 par la vitesse maximale souhaitée
    vel.linvel = direction * max_speed + player_vel.gravity;
    // if keys.pressed(KeyCode::Z) { // Walk
    //     p_vel += forward*speed;
    // }
    // if keys.pressed(KeyCode::S) { // Walk
    //     p_vel -= forward*speed;
    // }
    // if keys.pressed(KeyCode::Q) { // Walk
    //     p_vel -= right*speed;
    // }
    // if keys.pressed(KeyCode::D) { // Walk
    //     p_vel += right*speed;
    // }
    // let old_vel = player_vel.old_vel;
    // // println!("{}\t{}\t{}\t{}\t{:?}", player_vel.vel, player_vel.vel + player_vel.forward_vel,player_vel.forward_vel, vel.linvel - old_vel, vel.linvel);
    // player_vel.vel += vel.linvel - old_vel;
    // player_vel.vel *= 0.9;
    // vel.linvel = (player_vel.vel + player_vel.forward_vel + p_vel);
    // println!("{}\t= {}-{}\t{}\t{}", (vel.linvel - old_vel).round(), vel.linvel.round(), old_vel.round(), p_vel.round(), vel.linvel.round());
    // player_vel.old_vel = vel.linvel.clone();
    // strafe = strafe.clamp(-right, right);
    // player_force.force = (front + strafe) * 20.;
    // let max = Vec3::ONE*20.;
    // player_force.force = player_force.force.clamp(-max, max);


    // let max = Vec3::new(200.,300.,200.);
    // player_velo.movement_velo += vector_direction * SPEED * time.delta_seconds();
    // player_velo.movement_velo = player_velo.movement_velo.clamp(-max, max);
    // let max = Vec3::new(100.,100.,100.);
    // vel.linvel += Vec3::NEG_Y * 9.81 * time.delta_seconds();
    // player_velo.other_velo = player_velo.other_velo.clamp(-max, max);
    // vel.linvel = player_velo.movement_velo + player_velo.other_velo + (player_velo.old_velo - vel.linvel);
    camera.translation = pos.translation+Vec3::Y*1.75;
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
pub fn player_on_ground(
    mut player: Query<(&mut Player, &Transform)>,
    world: Res<World>,
) {
    let (mut player, pos) = player.single_mut();
    let (x,y,z) = (pos.translation.x,pos.translation.y,pos.translation.z);
    let y = if y - y.floor() < 0.375 { // y - y.floor() gets the decimal part (1.2 - 1 = 0.2)
        y.floor()
    } else {y.ceil()} as i32;
    player.on_ground = world.map.contains_key(&BlocPosition::new(x as i32,y-1,z as i32));
}


pub fn cursor_grab_system(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();
    if window.cursor.visible == false {
        window.set_cursor_position(Some(Vec2::new(450., 450.)));
    }
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.set_cursor_position(Some(Vec2::new(450., 450.)));
        window.cursor.visible = false;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}