use crate::game_state::GameState;
use crate::inventory::InventoryItem;
use crate::player::PlayerControllerState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::DebugRenderContext;

pub struct ItemSpawner;

impl Plugin for ItemSpawner {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                monitor_spawns.run_if(in_state(GameState::FightingInArena)),
                spawn_debug_items.run_if(in_state(GameState::FightingInArena)),
            ),
        );
    }
}

fn spawn_debug_items(
    mut context: ResMut<DebugRenderContext>,
    keys: Res<Input<KeyCode>>,
    player: Query<&Transform, With<PlayerControllerState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let item = if keys.just_pressed(KeyCode::Key1) {
        InventoryItem::from((
            (1, 3, 3),
            vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 2), (-2, 0, 2)],
            Color::rgba(1.0, 1.0, 1.0, 1.0),
        ))
    } else if keys.just_pressed(KeyCode::Key2) {
        InventoryItem::from((
            (5, 3, 2),
            vec![
                (0, 0, 0),
                (0, 0, 1),
                (0, 0, 2),
                (0, 0, 3),
                (0, 0, 4),
                (1, 0, 0),
                (-1, 0, 0),
                (0, 0, -1),
            ],
            Color::rgba(0.5, 0.5, 0.5, 1.0),
        ))
    } else if keys.just_pressed(KeyCode::Key3) {
        InventoryItem::from((
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
        ))
    } else {
        return;
    };

    let on_player = keys.pressed(KeyCode::ControlLeft);

    item.create_world_entity(
        player.single().translation,
        on_player,
        commands,
        meshes,
        materials,
    );
}

fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    create_boomerang(commands, meshes, materials);
}

fn create_boomerang(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let boomerang = InventoryItem::from((
        (1, 3, 3),
        vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
        Color::rgba(1.0, 1.0, 1.0, 1.0),
    ));

    boomerang.create_world_entity(
        Vec3 {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        },
        false,
        commands,
        meshes,
        materials,
    );
}

fn monitor_spawns() {}
