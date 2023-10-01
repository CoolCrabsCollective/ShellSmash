use crate::debug_camera_controller::DebugCameraControllerPlugin;
use crate::enemy::EnemyPlugin;
use crate::enemy_spawner::EnemySpawnerPlugin;
use crate::game_state::GameState;
use crate::inventory::InventoryItem;
use crate::item_spawner::ItemSpawner;
use crate::level_loader::{load_level, LevelLoaderPlugin};
use crate::player::PlayerPlugin;
use crate::world_item::ItemAttachmentPlugin;
use bevy::math::vec3;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(GameState::FightingInArena), reset_camera);
        app.add_plugins((
            LevelLoaderPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default().disabled(),
            PlayerPlugin,
            EnemyPlugin,
            EnemySpawnerPlugin,
            ItemSpawner,
            ItemAttachmentPlugin,
        ))
        .add_systems(Update, debug_render_toggle)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 });

        app.add_plugins(DebugCameraControllerPlugin);
    }
}

fn debug_render_toggle(mut context: ResMut<DebugRenderContext>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::F12) {
        context.enabled = !context.enabled;
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    load_level("map.glb#Scene0", &mut commands, &asset_server);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-0.75 * PI)),
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
        transform: get_camera_position(),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("enemy.glb#Scene0"),
        transform: Transform::from_xyz(5.0, 0.0, -5.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(vec3(0.25, 0.25, 0.25)),
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
}

fn get_camera_position() -> Transform {
    // DON'T CHANGE THE FOLLOWING LINE UNLESS YOU WANT TO DIE
    Transform::from_xyz(0.0, 30.0, 15.0).looking_at(vec3(0.0, 0.0, 2.0), Vec3::Y)
}

fn reset_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    (*camera_query.single_mut()) = get_camera_position();
}
