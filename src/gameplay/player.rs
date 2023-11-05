

use bevy::{prelude::*, input::mouse::MouseMotion, time::Time, window::{CursorGrabMode, PrimaryWindow}};
use bevy_rapier3d::prelude::{Velocity, RigidBody, Collider, ExternalImpulse, ExternalForce, RapierConfiguration, LockedAxes, Ccd, Damping, Restitution, Friction};
use bevy_tnua::{TnuaRapier3dIOBundle, prelude::{TnuaControllerBundle, TnuaController, TnuaBuiltinWalk, TnuaBuiltinJump}, TnuaRapier3dSensorShape};
use crate::{world::World, camera::CameraTag, bloc::BlocPosition};

use super::mobs::Living;

#[derive(Resource)]
pub struct JumpTimer(pub Timer);


#[derive(Component, Default)]
pub struct Player {
    pub name: String,
    pub on_ground: bool,
    pub sensitivity: f32,
    pub vel: PlayerVelocity,
    pub speed: f32,
    pub max_speed: f32,
    pub height: f32,
    pub render_distance: i32,
}
impl Player {
    pub fn new() -> Self {
        Self { 
            name: "Sxmourai".to_string(), 
            sensitivity: 0.00005,
            on_ground: true,
            speed: 1.0,
            max_speed: 5.0,
            height: 1.75,
            render_distance: 2,
            ..default()
        }
    }
}
#[derive(Default)]
pub struct PlayerVelocity {
    pub gravity: Vec3,
}

pub fn player_movement(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<CameraTag>>,
    mut player_query: Query<(&mut TnuaController, &mut Player, &Transform), Without<CameraTag>>,
    keys: Res<Input<KeyCode>>,
    time: ResMut<Time>,
    mut motion_evr: EventReader<MouseMotion>,
    mut timer: ResMut<JumpTimer>,
    _physic_config: Res<RapierConfiguration>,
) {
    let mut camera = camera_query.single_mut();
    let (mut controller,mut player, pos) = player_query.single_mut();
    let mut forward = camera.rotation.mul_vec3(Vec3::NEG_Z);
    let mut right = camera.rotation.mul_vec3(Vec3::NEG_X);
    forward.y = 0.;right.y = 0.;
    forward = forward.normalize();
    right = right.normalize();
    let mut direction = Vec3::ZERO;
    player.vel.gravity += Vec3::NEG_Y * 1.5;
    if player.on_ground {
        player.vel.gravity = Vec3::ZERO;
    }
    if keys.pressed(KeyCode::Z) {
        direction += forward*player.speed;
    }
    if keys.pressed(KeyCode::S) {
        direction -= forward*player.speed;
    }
    if keys.pressed(KeyCode::Q) {
        direction += right*player.speed;
    }
    if keys.pressed(KeyCode::D) {
        direction -= right*player.speed;
    }
    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction * 10.0,
        desired_forward: direction,
        float_height: 0.9,
        ..default()
    });
    if keys.pressed(KeyCode::Space) {
        println!("AA");
        controller.action(TnuaBuiltinJump {
            // The full height of the jump, if the player does not release the button:
            height: 4.0,
            shorten_extra_gravity: 0.0,
            allow_in_air: true,
            // input_buffer_time: 0.3,

            // TnuaBuiltinJump too has other fields that can be configured:
            ..Default::default()
        });
        println!("{:?}", controller.action_flow_status())
    }
    // timer.0.tick(time.delta());
    // if keys.pressed(KeyCode::Space) && player.on_ground { // Jump
    //     if timer.0.finished() {
    //         player.vel.gravity = Vec3::Y*15.;
    //         timer.0.reset();
    //     }
    // }

    // vel.linvel = direction * player.max_speed + player.vel.gravity;
    camera.translation = pos.translation+Vec3::Y*player.height;
    let window = primary_window.single();
    for ev in motion_evr.iter() {
        let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
        match window.cursor.grab_mode {
            CursorGrabMode::None => (),
            _ => {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                pitch -= (player.sensitivity * ev.delta.y * window_scale).to_radians();
                yaw -= (player.sensitivity * ev.delta.x * window_scale).to_radians();
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
    let y = if y - y.floor() < 0.38 { // y - y.floor() gets the decimal part (1.2 - 1 = 0.2)
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

pub fn spawn_player(commands: &mut Commands, pos: Transform) {
    let _player_height = 1.75;
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::new(20., 5., 20.), Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection { fov: 89., ..default()}),
        ..default()
    },CameraTag));


    let _player_id = commands.spawn((
        TransformBundle::from_transform(pos),
        Collider::cuboid(0.3, 1.75/2., 0.2),
        RigidBody::Dynamic,
        // Friction::coefficient(0.0),
        // Restitution::coefficient(0.),
        // Damping {
        //     linear_damping: 0.0,
        //     ..default()
        // },
        LockedAxes::ROTATION_LOCKED,
        Player::new(),
        // Ccd::enabled(),
        ExternalImpulse {
            impulse: Vec3::Y * 0.,
            ..default()
        },
        Living,
        TnuaControllerBundle::default(),
        // TnuaRapier3dSensorShape(Collider::cuboid(0.25, 1.7/2., 0.15)), // Make it smaller (https://docs.rs/bevy-tnua/latest/bevy_tnua/)
        TnuaRapier3dIOBundle::default(),// this one depends on the physics backend
    ))
    .id()
    ;
}

pub fn text_update(mut texts: Query<&mut Text>, player: Query<&Transform, With<Player>>) {
    let pos = player.single().translation;
    for mut text in texts.iter_mut() {
        if text.sections[0].value.starts_with("Coords: ") {
            text.sections[0].value = format!("Coords: {} = {:?}", pos, (pos.x as i32, pos.y as i32, pos.z as i32));
        }
    }
}