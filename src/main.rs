mod camera_controller;
mod inventory;
mod voxel_renderer;

use crate::camera_controller::CameraControllerPlugin;
use crate::inventory::InventoryItem;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use voxel_renderer::VoxelRendererPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }
                .into(),
            }),
            WireframePlugin,
            CameraControllerPlugin,
            VoxelRendererPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
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
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    let boomerang = InventoryItem::from((
        (0, 0, 0),
        vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
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
    ));

    boomerang.spawn_cubes(&mut commands, &mut meshes, &mut materials);
    sword.spawn_cubes(&mut commands, &mut meshes, &mut materials);
    heart.spawn_cubes(&mut commands, &mut meshes, &mut materials);
}
