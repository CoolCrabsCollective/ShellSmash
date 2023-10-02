use crate::game_state::GameState;
use crate::inventory::ItemType::{MELEE_WEAPON, NON_WEAPON, RANGED_WEAPON};
use crate::inventory::{InventoryItem, ItemTypeId};
use crate::player::PLAYER_HEIGHT;
use crate::world_item::{WeaponHolder, VOXEL_SIZE_IN_WORLD};
use bevy::math::vec3;
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
    context: ResMut<DebugRenderContext>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&Transform, &mut WeaponHolder)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut item = if keys.just_pressed(KeyCode::Key1) {
        let mut item = InventoryItem::from((
            (1, 3, 3),
            vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 2), (-2, 0, 2)],
            Color::rgba(1.0, 1.0, 1.0, 1.0),
            RANGED_WEAPON,
            ItemTypeId::Boomerang,
        ));
        item.weapon_attack_speed = 10.0;
        item.projectile_speed = 30.0;
        item
    } else if keys.just_pressed(KeyCode::Key2) {
        InventoryItem::from((
            (5, 3, 2),
            vec![
                (0, 0, 0),
                (0, 0, 1),
                (0, 0, 2),
                (0, 0, 3),
                (0, 0, 4),
                (0, 0, 5),
                (0, 0, 6),
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, -1),
            ],
            Color::rgba(0.5, 0.5, 0.5, 1.0),
            MELEE_WEAPON,
            ItemTypeId::AlexSword,
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
            NON_WEAPON,
            ItemTypeId::Heart,
        ))
    } else {
        return;
    };

    let on_player = keys.pressed(KeyCode::ControlLeft);

    let entity = item.create_world_entity(
        player.single().0.translation
            + if on_player {
                Vec3::splat(0.0)
            } else {
                vec3(0.0, -PLAYER_HEIGHT + VOXEL_SIZE_IN_WORLD, 0.0)
            },
        on_player,
        false,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    if on_player {
        player.single_mut().1.current_weapon = Some((entity, item));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // create_boomerang(commands, meshes, materials);
    create_sword(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
    );
    create_david_gun(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3 {
            x: 5.0,
            y: 0.5,
            z: 8.0,
        },
    );
}

// fn create_boomerang(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let boomerang = InventoryItem::from((
//         (1, 3, 3),
//         vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
//         Color::rgba(1.0, 1.0, 1.0, 1.0),
//         RANGED_WEAPON,
//     ));
//
//     boomerang.create_world_entity(
//         Vec3 {
//             x: 0.0,
//             y: 0.5,
//             z: 0.0,
//         },
//         false,
//         true,
//         commands,
//         meshes,
//         materials,
//     );
// }

fn create_david_gun(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    location: Vec3,
) {
    let mut gun = InventoryItem::from((
        (1, 0, 3),
        vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 2), (-2, 0, 2)],
        Color::rgba(1.0, 1.0, 1.0, 1.0),
        RANGED_WEAPON,
        ItemTypeId::DavidGun,
    ));
    gun.weapon_attack_speed = 10.0;
    gun.projectile_speed = 30.0;

    gun.create_world_entity(location, false, true, commands, meshes, materials);
}

fn create_sword(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    location: Vec3,
) {
    let sword = InventoryItem::from((
        (5, 0, 2),
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, -1),
        ],
        Color::rgba(0.0, 1.0, 0.0, 1.0),
        MELEE_WEAPON,
        ItemTypeId::WillSword,
    ));

    sword.create_world_entity(location, false, true, commands, meshes, materials);
}

fn monitor_spawns() {}
