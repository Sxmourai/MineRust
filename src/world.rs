use bevy::{prelude::*, utils::{HashMap, HashSet}};
use bevy_rapier3d::prelude::{Collider, Friction, Restitution};
use noise::{NoiseFn, Perlin};

use crate::{Player, bloc::{BlocPosition, Bloc}};

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
    // pub fn generate(&mut self, 
    //     commands: &mut Commands,
    //     meshes: &mut ResMut<Assets<Mesh>>,
    //     asset_server: &Res<AssetServer>,
    //     materials: &mut ResMut<Assets<StandardMaterial>>,
    // ) {
    //     let texture_handle = asset_server.load("minecraft/textures/block/dirt.png");
    //     let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    //     let material = materials.add(StandardMaterial {
    //         base_color_texture: Some(texture_handle.clone()),
    //         // alpha_mode: AlphaMode::Blend,
    //         // unlit: true,
    //         ..default()
    //     });
    //     let mut entities = Vec::new();
    //     for (pos, _bloc) in self.map.iter() {
    //         if is_bloc_at_surface(*pos, &self) {
    //             if let Some(bloc) = self.map.get(pos) {
    //                 if bloc.id.is_some() {continue} // Check if already spawned
    //             }
    //             let mut bundle = commands.spawn(PbrBundle {
    //                 mesh: cube.clone(),
    //                 material: material.clone(),
    //                 transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
    //                 ..default()
    //             });
    //             let entity = bundle
    //             .insert(bevy_rapier3d::prelude::RigidBody::Fixed)
    //             .insert(Friction::coefficient(0.0))
    //             .insert(Restitution::coefficient(0.))
    //             // .insert(Sleeping {sleeping:false,..default()})
    //             // .insert(Collider::cuboid(0.5, 0.5, 0.5)) // Cube dimensions
    //             ;
    //             entities.push((*pos, entity.id()));
    //             self.entities.insert(entity.id(), *pos);
    //         }
    //     }
    //     for (pos, entity) in entities {
    //         self.map.get_mut(&pos).unwrap().id = Some(entity);
    //     }
    // }
    // pub fn setup(&mut self, 
    //     mut commands: Commands,
    //     mut meshes: ResMut<Assets<Mesh>>,
    //     asset_server: Res<AssetServer>,
    //     mut materials: ResMut<Assets<StandardMaterial>>,
    //     player_transform: &Transform
    // ) {
    //     for x in 0..CHUNK_SIZE*2 {
    //         for z in 0..CHUNK_SIZE*2 {
    //             for y in 0..=self.get_height_at(x as f64, z as f64) as i64 {
    //                 let pos = BlocPosition::new(x as i32, y as i32, z as i32);
    //                 self.map.insert(pos, Bloc::new(pos, None));
    //             }
    //         }
    //     }
    //     self.generate(&mut commands, &mut meshes, &asset_server, &mut materials);
    // }
    pub fn generate_chunk(&mut self, 
        cx: i32, cz: i32,
        commands: &mut Commands,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
) {
    let mix = cx as isize * CHUNK_SIZE as isize;
    let max = (cx as isize+1) * CHUNK_SIZE as isize;
    let miz = cz as isize * CHUNK_SIZE as isize;
    let maz = (cz as isize+1) * CHUNK_SIZE as isize;
    let mut entities = Vec::new();
    for x in mix..max {
        for z in miz..maz {
            for y in 0..=self.get_height_at(x as f64, z as f64) as i64 {
                let pos = BlocPosition::new(x as i32, y as i32, z as i32);
                if let Some(bloc) = self.map.get(&pos) {
                    if bloc.id.is_some() {continue} // Check if already spawned
                }
                self.map.insert(pos, Bloc::new(pos, None));
                if !is_bloc_at_surface(pos, self) {continue}
                // Bloc is visible, so render it
                let entity = commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                    ..default()
                })
                .insert(bevy_rapier3d::prelude::RigidBody::Fixed)
                .insert(Friction::coefficient(0.0))
                .insert(Restitution::coefficient(0.))
                .id()
                ;
                entities.push((pos, entity)); // Push to then add entities to block
                // Can't do it in one loop because it would be using 2 mut pointers to world :c
            }
        }
    }
    for (pos, entity) in entities {
        self.map.get_mut(&pos).unwrap().id = Some(entity);
        self.entities.insert(entity, pos);
    }
    }
}


pub fn gen_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<World>,
    player_pos: Query<(&Transform,&Player)>,
) {
    let texture_handle = asset_server.load("minecraft/textures/block/dirt.png");
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        ..default()
    });
    let (pos, player) = player_pos.single();
    let (px, pz) = (pos.translation.x as i32 / CHUNK_SIZE as i32, pos.translation.z as i32 / CHUNK_SIZE as i32);
    for chunk_x in px-player.render_distance..px+player.render_distance {
        for chunk_z in pz-player.render_distance..pz+player.render_distance {
            if world.generated_chunks.get(&IVec2::new(chunk_x, chunk_z)).is_none() {
                world.generate_chunk(chunk_x, chunk_z, &mut commands, cube.clone(), material.clone());
            }
        }
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
    player: Query<&Transform, With<Player>>,
) {
    let (px,py,pz) = {let pos = player.single().translation; (pos.x as i32, pos.y as i32, pos.z as i32)};
    let mut pn = [(0,0,0); 9*4];
    for y in 0..4 {
        for x in 0..3 {
            for z in 0..3 {
                pn[y*9 + x*3 + z] = (px+x as i32 - 1, py+y as i32 - 1, pz+z as i32 - 1);
                // println!("{} -> {:?}", y*9 + x*3 + z, (x as i32 - 1, y as i32 - 1, z as i32 - 1))
            }
        }
    }
    let mut to_remove = Vec::new();
    for pos in world.collision_map.iter() {
        let p = (pos.x, pos.y, pos.z);
        if !(pn.contains(&p)) {
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
    for (x,y,z) in pn {
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