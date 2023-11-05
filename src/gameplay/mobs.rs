use bevy::{prelude::{Vec3, Component, Query, Transform, ResMut, Commands, Resource, Res, Assets, StandardMaterial, PbrBundle, default, shape::{Cube, Box}, Mesh}, time::{Timer, Time}, sprite::{SpriteBundle, Sprite}};
use bevy_rapier3d::prelude::{Collider, RigidBody, Friction, Restitution, LockedAxes, ExternalImpulse, Velocity};

use crate::world::World;

#[derive(Component)]
pub struct Mob {
    pub health: usize,
}

pub struct Sheep {
    pub health: usize
}
pub fn animal_live(mut animals: Query<(&mut Mob, &mut Velocity, &mut MoveTimer)>, time: Res<Time>) {
    for (animal,mut vel, mut timer) in animals.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            vel.linvel.x = alea::f32_in_range(-1.1, 1.1);
            vel.linvel.z = alea::f32_in_range(-1.1, 1.1);
            timer.0.reset()
        }
    }
}

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct MoveTimer(pub Timer);
#[derive(Component)]
pub struct Living;

pub fn spawn_animals(
    mut spawn_timer: ResMut<SpawnTimer>, 
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    world: Res<World>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(Box::new(1., 1., 2.))),
            material: materials.add(StandardMaterial { 
                base_color: bevy::prelude::Color::Rgba { red: 255., green: 0., blue: 0., alpha: 1.0 },
                ..default()
            }),
            transform: Transform::from_xyz(10. as f32, world.get_height_at(10., 10.) as f32+1., 10. as f32),
            ..default()
            },
            Mob {health: 10},
            Collider::cuboid(0.5, 0.5, 1.0),
            RigidBody::Dynamic,
            Friction::coefficient(0.),
            Restitution::coefficient(0.),
            LockedAxes::ROTATION_LOCKED,
            ExternalImpulse {
                impulse: Vec3::Y * 0.,
                ..default()
            },
            Velocity::default(),
            MoveTimer(Timer::from_seconds(2.0, bevy::time::TimerMode::Once)),
            Living,
        ));
        spawn_timer.0.reset()
    }
}