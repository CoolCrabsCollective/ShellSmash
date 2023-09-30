use std::f32::consts::PI;

use bevy::math::vec3;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use debug_camera_controller::DebugCameraControllerPlugin;

use crate::inventory::{move_inventory_items, update_inventory_data, InventoryData, InventoryItem};
use crate::inventory_controller::InventoryControllerPlugin;
use crate::master_controller::MasterControllerPlugin;
use crate::voxel_renderer::VoxelRendererPlugin;

use bevy_rapier3d::prelude::NoUserData;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use level_loader::load_level;
use level_loader::LevelLoaderPlugin;

mod debug_camera_controller;
mod inventory;
mod inventory_controller;
mod level_loader;
mod master_controller;
mod math;
mod voxel_renderer;
mod wall;

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
            MasterControllerPlugin,
            InventoryControllerPlugin,
            // CameraControllerPlugin,
            VoxelRendererPlugin,
            LevelLoaderPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
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
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    load_level("map.glb#Scene0", &asset_server);
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
        transform: Transform::from_xyz(0.0, 35.0, -15.0).looking_at(vec3(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("map.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_rotation(Quat::from_rotation_y(0.5 * PI)),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("player.glb#Scene0"),
        transform: Transform::from_xyz(5.0, 0.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(vec3(0.5, 0.5, 0.5)),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("enemy.glb#Scene0"),
        transform: Transform::from_xyz(5.0, 0.0, -5.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(vec3(0.5, 0.5, 0.5)),
        ..default()
    });

    let boomerang = InventoryItem::from((
        (1, 3, 3),
        vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
        Color::rgba(1.0, 1.0, 1.0, 1.0),
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
        Color::rgba(0.0, 1.0, 0.0, 1.0),
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
        Color::rgba(1.0, 0.0, 0.0, 1.0),
    ));

    commands.spawn(boomerang);
    commands.spawn(sword);
    commands.spawn(heart);
    commands.insert_resource(InventoryData { grid: Vec::new() });
}
