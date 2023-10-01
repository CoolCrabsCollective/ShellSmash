use crate::game_state::GameState;
use crate::inventory::InventoryItem;
use bevy::prelude::*;

pub struct ItemSpawner;

impl Plugin for ItemSpawner {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            monitor_spawns.run_if(in_state(GameState::FightingInArena)),
        );
    }
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
        commands,
        meshes,
        materials,
    );
}

fn monitor_spawns() {}
