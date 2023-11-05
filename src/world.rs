use bevy::{prelude::*, utils::{HashMap, HashSet}};
use bevy_rapier3d::prelude::{Collider, Friction, Restitution};
use noise::{NoiseFn, Perlin};

use crate::{Player, bloc::{BlocPosition, Bloc}, gameplay::mobs::Living};

pub type ChunkPosition = IVec2; // Vec3 because a chunk is full height
const CHUNK_SIZE: usize = 16;

#[derive(Resource, Default)]
pub struct World {
    pub map: HashMap<BlocPosition, Bloc>,
    pub seed: u32,
    pub perlin: Perlin,
    pub entities: HashMap<Entity, BlocPosition>,
    pub collision_map: HashSet<BlocPosition>,
    pub generated_chunks: HashSet<ChunkPosition>
}
impl World {
    pub fn new(seed: u32) -> Self {
        Self { 
            seed,
            perlin: Perlin::new(seed),
            ..default()
        }
    }
    pub fn get_height_at(&self, x: f64, z: f64) -> f64 {
        return (self.perlin.get([x / (CHUNK_SIZE*8+1) as f64,z / (CHUNK_SIZE*8+1) as f64])*20.).round().abs();
    }

//     pub fn generate_chunk(&mut self, 
//         cx: i32, cz: i32,
//         bevy_world: &mut bevy::prelude::World,
//         mesh: Handle<Mesh>,
//         material: Handle<StandardMaterial>,
// ) {
//     }
}


pub fn gen_world(
    bevy_world: &mut bevy::prelude::World,
) {
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut world: ResMut<World>,
    // player_pos: Query<(&Transform,&Player)>,
    let asset_server = bevy_world.resource::<AssetServer>();
    let texture_handle = asset_server.load("minecraft/textures/block/dirt.png");
    let mut materials = bevy_world.resource_mut::<Assets<StandardMaterial>>();
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        ..default()
    });
    let mut meshes = bevy_world.resource_mut::<Assets<Mesh>>();
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let (pos, player) = bevy_world.query::<(&Transform,&Player)>().single(&bevy_world);
    let render_distance = player.render_distance.clone();
    let (px, pz) = (pos.translation.x as i32 / CHUNK_SIZE as i32, pos.translation.z as i32 / CHUNK_SIZE as i32);
    let mut world = bevy_world.resource_mut::<World>();
    let mut cubes = Vec::new();
    for chunk_x in px-render_distance..px+render_distance {
        for chunk_z in pz-render_distance..pz+render_distance {
            if world.generated_chunks.get(&IVec2::new(chunk_x, chunk_z)).is_none() {
                let mix = chunk_x as isize * CHUNK_SIZE as isize;
                let max = (chunk_x as isize+1) * CHUNK_SIZE as isize;
                let miz = chunk_z as isize * CHUNK_SIZE as isize;
                let maz = (chunk_z as isize+1) * CHUNK_SIZE as isize;
                for x in mix..max {
                    for z in miz..maz {
                        for y in 0..=world.get_height_at(x as f64, z as f64) as i64 {
                            let pos = BlocPosition::new(x as i32, y as i32, z as i32);
                            if let Some(bloc) = world.map.get(&pos) {
                                if bloc.id.is_some() {continue} // Check if already spawned
                            }
                            world.map.insert(pos, Bloc::new(pos, None));
                            if !is_bloc_at_surface(pos, &world) {continue}
                            // Bloc is visible, so render it
                            
                            cubes.push((PbrBundle {
                                mesh: cube.clone(),
                                material: material.clone(),
                                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                                ..default()
                            },bevy_rapier3d::prelude::RigidBody::Fixed,Friction::coefficient(0.0),Restitution::coefficient(0.)
                            ))
                            ;
                            // entities.push((pos, entity)); // Push to then add entities to block
                            // Can't do it in one loop because it would be using 2 mut pointers to world :c
                        }
                    }
                }
            }
        }
    }
    let entities = bevy_world.spawn_batch(cubes);
    let mut spawned = Vec::new();
    for entity in entities.collect::<Vec<Entity>>() {
        let pos = bevy_world.get::<Transform>(entity).unwrap().translation;
        spawned.push((BlocPosition::new(pos.x as i32,pos.y as i32,pos.z as i32), entity));
    }
    let mut world = bevy_world.resource_mut::<World>();
    for (pos, entity) in spawned {
        world.map.get_mut(&pos).unwrap().id = Some(entity);
        world.entities.insert(entity, pos);
    }
}

fn is_bloc_at_surface(pos: BlocPosition, world: &World) -> bool {
    let (x,y,z) = (pos.x, pos.y, pos.z);
    let neighbors = [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ];
    for &(nx, ny, nz) in &neighbors {
        if !world.map.contains_key(&BlocPosition::new(nx,ny,nz)) {
            return true;
        }
    }
    return false;
}
pub fn is_bloc_visible(pos: BlocPosition, world: &World, view_projection_matrix: Mat4) -> bool {
    if is_bloc_at_surface(pos, world) {
        let fpos = Vec3::new(pos.x as f32,pos.y as f32,pos.z as f32);
        let transformed_position = view_projection_matrix * fpos.extend(1.0);
        return transformed_position.x >= -1.0 && transformed_position.x <= 1.0 &&
            transformed_position.y >= -1.0 && transformed_position.y <= 1.0 &&
            transformed_position.z >= -1.0 && transformed_position.z <= 1.0
    }
    false
}
pub fn optimise_world(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<World>,
    livings: Query<&Transform, With<Living>>,
) {
    let mut livings_neighbors = Vec::new();
    for living in livings.iter() {
        let (px,py,pz) = {let pos = living.translation; (pos.x as i32, pos.y as i32, pos.z as i32)};
        
        for y in 0..4 {
            for x in 0..3 {
                for z in 0..3 {
                    livings_neighbors.push((px+x as i32 - 1, py+y as i32 - 1, pz+z as i32 - 1));
                    // println!("{} -> {:?}", y*9 + x*3 + z, (x as i32 - 1, y as i32 - 1, z as i32 - 1))
                }
            }
        }
    }
    
    let mut to_remove = Vec::new();
    for pos in world.collision_map.iter() {
        let p = (pos.x, pos.y, pos.z);
        if !(livings_neighbors.contains(&p)) {
            to_remove.push(pos.clone())
        }
    }
    for p in to_remove {
        world.collision_map.remove(&p);
        if let Some(bloc) = world.map.get(&p) {
            if let Some(entity) = bloc.id {
                commands.get_entity(entity).unwrap().remove::<Collider>();
            } else {
                println!("Bloc at {:?} exists but not spawned", p);
            }
        } else {
            println!("Bloc at {:?} isn't in map", p);
        }
    }
    for (x,y,z) in livings_neighbors {
        let pos = IVec3::new(x,y,z);
        
        if let Some(bloc) = world.map.get(&pos) { // Not sure there's a bloc, if the player is in the air
            if let Some(entity) = bloc.id { // Not sure he has an entity, if he isn't at surface he isn't rendered
                world.collision_map.insert(pos);
                commands.get_entity(entity).unwrap()
                .insert(Collider::cuboid(0.5, 0.5, 0.5));
                // commands.get_entity(entity).unwrap().log_components();
            }
        }
    }
}