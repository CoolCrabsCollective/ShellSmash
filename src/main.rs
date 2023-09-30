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
use crate::voxel_renderer::VoxelRendererPlugin;

use bevy_rapier3d::prelude::{Collider, NoUserData, RigidBody};
use bevy_rapier3d::prelude::{ColliderMassProperties, Friction, RapierPhysicsPlugin, Restitution};
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use level_loader::load_level;
use level_loader::LevelLoaderPlugin;

mod debug_camera_controller;
mod inventory;
mod inventory_controller;
mod item_mesh_generator;
mod level_loader;
mod math;
mod voxel_renderer;
mod wall;

const USE_DEBUG_CAM: bool = false;
const SPAWN_PACKING_SHIT: bool = false;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Game,
    Inventory,
}

impl GameState {
    fn turn(&self) -> Self {
        match self {
            GameState::Game => GameState::Inventory,
            GameState::Inventory => GameState::Game,
        }
    }
}

// add physics
fn main() {
    let mut app = App::new();
    app.insert_resource(AmbientLight {
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
        LevelLoaderPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
    ))
    .add_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(Update, (bevy::window::close_on_esc, swap_controls));

    if SPAWN_PACKING_SHIT {
        app.add_plugins((WireframePlugin, VoxelRendererPlugin));
        app.add_systems(Update, (move_inventory_items, update_inventory_data));
    }

    if USE_DEBUG_CAM {
        app.add_plugins(DebugCameraControllerPlugin);
    } else if SPAWN_PACKING_SHIT {
        app.add_plugins(InventoryControllerPlugin);
    }

    app.run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    load_level("map.glb#Scene0", &mut commands, &asset_server);
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
        // IF YOU CHANGE THIS YOU DIE DIPSHIT
        transform: Transform::from_xyz(0.0, 35.0, -15.0).looking_at(vec3(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("map.glb#Scene0"),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0)
    //         .looking_at(Vec3::ZERO, Vec3::Y)
    //         .with_rotation(Quat::from_rotation_y(0.5 * PI)),
    //     ..default()
    // });

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

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(1.0, 0.25))
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 5.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0));

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

    commands.spawn(PbrBundle {
        mesh: meshes.add(boomerang.generate_mesh()),
        material: materials.add(StandardMaterial {
            base_color: boomerang.color,
            ..default()
        }),
        transform: Transform::from_xyz(10.0, 1.0, 3.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(sword.generate_mesh()),
        material: materials.add(StandardMaterial {
            base_color: sword.color,
            ..default()
        }),
        transform: Transform::from_xyz(10.0, 11.0, 3.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(heart.generate_mesh()),
        material: materials.add(StandardMaterial {
            base_color: heart.color,
            ..default()
        }),
        transform: Transform::from_xyz(10.0, 21.0, 3.0),
        ..default()
    });

    if SPAWN_PACKING_SHIT {
        commands.spawn(boomerang);
        commands.spawn(sword);
        commands.spawn(heart);
    }
    commands.insert_resource(InventoryData { grid: Vec::new() });
}

fn swap_controls(
    k_input: Res<Input<KeyCode>>,
    current_game_state: ResMut<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if k_input.just_pressed(KeyCode::Space) {
        let new_status = current_game_state.get().turn();

        next_game_state.set(new_status);
    }
}
