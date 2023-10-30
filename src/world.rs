use bevy::{prelude::*, utils::HashMap};
use noise::{NoiseFn, Perlin, Seedable};

type BlocPosition = (i64, u16, i64);
#[derive(Component)]
pub struct Bloc {
    pub pos: BlocPosition,
}
impl Bloc {
    pub fn new(pos: BlocPosition) -> Self {
        Self { 
            pos
        }
    }
}

#[derive(Resource)]
pub struct WorldMap(pub HashMap<BlocPosition, Bloc>);


const CHUNK_SIZE: usize = 32;
const CHUNKS: usize = 2;

pub fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<WorldMap>,
) {
    let perlin = Perlin::new(1);
    for x in 0..CHUNK_SIZE*CHUNKS {
        for z in 0..CHUNK_SIZE*CHUNKS {
            let height = (perlin.get([x as f64 / (CHUNK_SIZE*2+1) as f64,z as f64 / (CHUNK_SIZE*2+1) as f64])*100.).round().abs();
            for y in 0..height as i64 {
                let pos = (x as i64, y as u16, z as i64);
                world.0.insert(pos, Bloc::new(pos));
            }
            let visibility = Visibility::Visible;
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(x as f32, height as f32, z as f32),
                visibility,
                ..default()
            });
        }
    }
    for (pos, _bloc) in world.0.iter() {
        let (x,y,z) = *pos;
        if is_bloc_visible(*pos, &world) {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                ..default()
            });
        }
    }
}
fn is_bloc_visible(pos: BlocPosition, world: &WorldMap) -> bool {
    let (x,y,z) = pos;
    let neighbors = [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        // (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ];
    for &(nx, ny, nz) in &neighbors {
        if !world.0.contains_key(&(nx,ny,nz)) {
            return true; // Le voisin n'est pas présent
        }
    }
    if y > 0 && !world.0.contains_key(&(x, y - 1, z)) {
        return true;
    }
    return false;
}