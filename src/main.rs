use std::f32::consts::PI;

use bevy::input::keyboard::KeyCode;
use bevy::math::vec3;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;

use voxel_renderer::GRID_DIMS;

use crate::inventory::InventoryData;
use crate::inventory::InventoryItem;
use crate::inventory_controller::InventoryControllerPlugin;
use crate::voxel_renderer::VoxelRendererPlugin;

mod inventory;
mod inventory_controller;
mod math;
mod voxel_renderer;

// add physics
fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                },
            }),
            WireframePlugin,
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
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,

            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-0.25 * PI)),
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-30.0, 20.0, 0.0).looking_at(vec3(-5.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("map.glb#Scene0"),
        ..default()
    });

    let boomerang = InventoryItem::from((
        (1, 3, 3),
        vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
        bevy::render::color::Color::rgba(1.0, 1.0, 1.0, 1.0),
    ));
    let sword = InventoryItem::from((
        (5, 3, 2),
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
        (2, 5, 2),
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
    inv.grid = InventoryData::grid_from_items(items, IVec3::from_array(GRID_DIMS))
}

fn move_inventory_items(mut query: Query<&mut InventoryItem>, k_input: Res<Input<KeyCode>>) {
    for mut item in &mut query {
        if k_input.pressed(KeyCode::Left) {
            item.translate(IVec3 { x: 1, y: 1, z: 1 })
        }
    }
}
