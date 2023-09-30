mod inventory;
mod inventory_controller;
mod master_controller;
mod math;
mod voxel_renderer;

use crate::inventory::InventoryData;
use crate::inventory::InventoryItem;
use bevy::input::keyboard::KeyCode;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;

use crate::master_controller::MasterControllerPlugin;
use inventory_controller::InventoryControllerPlugin;
use voxel_renderer::VoxelRendererPlugin;

// add physics
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                },
            }),
            WireframePlugin,
            MasterControllerPlugin,
            InventoryControllerPlugin,
            VoxelRendererPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                move_inventory_items,
                update_inventory_data,
            ),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
    // cube
    //commands.spawn(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //    ..default()
    //});
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let boomerang = InventoryItem::from((
        (0, 0, 0),
        vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
        bevy::render::color::Color::rgba(1.0, 1.0, 1.0, 1.0),
    ));
    let sword = InventoryItem::from((
        (5, 0, 0),
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, -1),
        ],
        bevy::render::color::Color::rgba(0.0, 1.0, 0.0, 1.0),
    ));
    let heart = InventoryItem::from((
        (0, 5, 0),
        vec![
            (0, 0, 0),
            (0, 0, -1),
            (1, 0, 0),
            (-1, 0, 0),
            (-1, 0, 1),
            (1, 0, 1),
        ],
        bevy::render::color::Color::rgba(1.0, 0.0, 0.0, 1.0),
    ));

    // boomerang.spawn_cubes(&mut commands, &mut meshes, &mut materials);
    // sword.spawn_cubes(&mut commands, &mut meshes, &mut materials);
    // heart.spawn_cubes(&mut commands, &mut meshes, &mut materials);
    commands.spawn(boomerang);
    commands.spawn(sword);
    commands.spawn(heart);
    commands.insert_resource(InventoryData { grid: Vec::new() });
}

fn update_inventory_data(query: Query<&InventoryItem>, mut inv: ResMut<InventoryData>) {
    let mut items: Vec<InventoryItem> = Vec::new();
    for p in query.iter() {
        items.push(p.clone())
    }
    inv.grid = InventoryData::grid_from_items(items, IVec3 { x: 5, y: 5, z: 5 })
}

fn move_inventory_items(mut query: Query<&mut InventoryItem>, k_input: Res<Input<KeyCode>>) {
    for mut item in &mut query {
        if k_input.pressed(KeyCode::Left) {
            item.translate(IVec3 { x: 1, y: 1, z: 1 })
        }
    }
}
