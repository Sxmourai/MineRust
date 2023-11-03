use bevy::{prelude::*, utils::{HashMap, HashSet}};
use bevy_rapier3d::{prelude::{Velocity, Collider, Friction, Sleeping, Restitution, KinematicCharacterController}, rapier::prelude::ColliderBuilder};
use noise::{NoiseFn, Perlin};

use crate::setup::Player;

pub type BlocPosition = IVec3;
#[derive(Component, Debug)]
pub struct Bloc {
    pub pos: BlocPosition,
    pub id: Option<Entity>,
}
impl Bloc {
    pub fn new(pos: BlocPosition, id: Option<Entity>) -> Self {
        Self { 
            pos,
            id,
        }
    }
}
#[derive(Resource, Default)]
pub struct World {
    pub map: HashMap<BlocPosition, Bloc>,
    pub seed: u32,
    pub perlin: Perlin,
    pub entities: HashMap<Entity, BlocPosition>,
    pub collision_map: HashSet<BlocPosition>
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
    pub fn generate(&mut self, 
        view_projection_matrix: Mat4,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let texture_handle = asset_server.load("minecraft/textures/block/dirt.png");
        let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            // alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });
        let mut entities = Vec::new();
        for (pos, _bloc) in self.map.iter() {
            if is_bloc_at_surface(*pos, &self) {
                if let Some(bloc) = self.map.get(pos) {
                    if bloc.id.is_some() {continue} // Check if already spawned
                }
                let mut bundle = commands.spawn(PbrBundle {
                    mesh: cube.clone(),
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                    ..default()
                });
                let entity = bundle
                .insert(bevy_rapier3d::prelude::RigidBody::Fixed)
                .insert(Friction::coefficient(0.0))
                .insert(Restitution::coefficient(0.))
                // .insert(Sleeping {sleeping:false,..default()})
                // .insert(Collider::cuboid(0.5, 0.5, 0.5)) // Cube dimensions
                ;
                entities.push((*pos, entity.id()));
                self.entities.insert(entity.id(), *pos);
            }
        }
        for (pos, entity) in entities {
            self.map.get_mut(&pos).unwrap().id = Some(entity);
        }
    }
    pub fn setup(&mut self, 
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        camera_transform: &Transform, camera: &Camera
    ) {
        for x in 0..CHUNK_SIZE*CHUNKS {
            for z in 0..CHUNK_SIZE*CHUNKS {
                for y in 0..=self.get_height_at(x as f64, z as f64) as i64 {
                    let pos = BlocPosition::new(x as i32, y as i32, z as i32);
                    self.map.insert(pos, Bloc::new(pos, None));
                }
            }
        }
        let view_projection_matrix = camera.projection_matrix() * camera_transform.compute_matrix();
        self.generate(view_projection_matrix, &mut commands, &mut meshes, &asset_server, &mut materials);
    }
}

const CHUNK_SIZE: usize = 32;
const CHUNKS: usize = 4;

pub fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<World>,
    camera_query: Query<(&Transform, &Camera)>,
) {
    let (camera_transform, camera) = camera_query.single();
    world.setup(commands, meshes, asset_server, materials, camera_transform, camera);
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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