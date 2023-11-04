

use bevy::{prelude::*, input::mouse::MouseMotion, time::Time, window::{CursorGrabMode, PrimaryWindow}};
use bevy_rapier3d::prelude::{Velocity, RigidBody, Collider, ExternalImpulse, ExternalForce, RapierConfiguration, LockedAxes, Ccd, Damping, Restitution, Friction};
use crate::{world::World, camera::CameraTag, bloc::BlocPosition};

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
    mut player_query: Query<(&mut Velocity, &mut Player, &Transform), Without<CameraTag>>,
    keys: Res<Input<KeyCode>>,
    time: ResMut<Time>,
    mut motion_evr: EventReader<MouseMotion>,
    mut timer: ResMut<JumpTimer>,
    _physic_config: Res<RapierConfiguration>,
) {
    let mut camera = camera_query.single_mut();
    let (mut vel,mut player, pos) = player_query.single_mut();
    let mut forward = camera.rotation.mul_vec3(Vec3::NEG_Z);
    let mut right = camera.rotation.mul_vec3(Vec3::NEG_X);
    forward.y = 0.;right.y = 0.;
    forward = forward.normalize();
    right = right.normalize();
    let mut direction = Vec3::ZERO;
    
    player.vel.gravity += Vec3::NEG_Y * 0.5;
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
    timer.0.tick(time.delta());
    if keys.pressed(KeyCode::Space) && player.on_ground { // Jump
        if timer.0.finished() {
            player.vel.gravity = Vec3::Y*6.1; 
            timer.0.reset();
        }
    }
    if keys.pressed(KeyCode::ShiftLeft) { //TODO Sneak
    }
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    vel.linvel = direction * player.max_speed + player.vel.gravity;
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

pub fn spawn_player(commands: &mut Commands, pos: Transform) {
    let _player_height = 1.75;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::new(20., 5., 20.), Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection { fov: 89., ..default()}),
        ..default()
    }).insert(CameraTag);



    let _player_id = commands
    .spawn(TransformBundle::from_transform(pos))
    .insert(Collider::cuboid(0.3, 1.75/2., 0.2))
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
}