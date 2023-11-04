use bevy::prelude::*;
use crate::{world::World, gameplay::player::spawn_player};

pub fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<StandardMaterial>>,
    world: ResMut<World>,
) {
    // Sun
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10., 10., 10.),
        ..default()
    });
    let font = asset_server.load("fonts/Minecraft.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 24.0,
        color: Color::WHITE,
    };
    commands.spawn(TextBundle {
        text: Text::from_section("Coords: ", text_style).with_alignment(TextAlignment::Left),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
    
    let (cx,cz) = (5.,5.);
    let cy = world.get_height_at(cx, cz)+1.;
    println!("Spawning player ({}, {}, {})", cx,cy,cz);
    let transform = Transform::from_xyz(cx as f32, cy as f32, cz as f32);
    spawn_player(&mut commands, transform);
}