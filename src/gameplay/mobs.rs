use bevy::{prelude::{Vec3, Component, Query, Transform, ResMut, Commands, Resource, Res}, time::{Timer, Time}, sprite::{SpriteBundle, Sprite}};

#[derive(Component)]
pub struct Mob {
    pub health: usize,
}

pub struct Sheep {
    pub health: usize
}
pub fn animal_live(mut animals: Query<(&mut Mob, &mut Transform)>) {
    for animal in animals.iter_mut() {

    }
}
#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn spawn_animals(
    mut spawn_timer: ResMut<SpawnTimer>, 
    mut commands: Commands,
    time: Res<Time>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        // commands.spawn(SpriteBundle {
            
        //     ..default()
        // })
        spawn_timer.0.reset()
    }
}