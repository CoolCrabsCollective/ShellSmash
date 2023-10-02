use bevy::prelude::*;
use bevy::prelude::*;
use queues::{IsQueue, Queue};
use rand::{random, Rng};

use crate::inventory::ItemType::{MELEE_WEAPON, NON_WEAPON, RANGED_WEAPON};
use crate::inventory::{InventoryItem, ItemTypeId};
use crate::wave_manager::ARENA_DIMENSIONS_METERS;

pub struct ItemSpawner;

impl Plugin for ItemSpawner {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_sword(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3 {
            x: -3.0,
            y: 0.5,
            z: 0.0,
        },
    );
    create_heart(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3 {
            x: 5.0,
            y: 0.5,
            z: -8.0,
        },
    );
}

fn create_alex_boomerang_copyrighted_you_need_permissions_to_use(
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
        ItemTypeId::AlexBoomerang,
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

fn create_better_sword(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    location: Vec3,
) {
    let mut sword = InventoryItem::from((
        (0, 0, 0),
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (0, 0, 3),
            (0, 0, 4),
            (0, 0, 5),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, -1),
        ],
        Color::rgba(0.5, 0.5, 0.5, 1.0),
        MELEE_WEAPON,
        ItemTypeId::MidSword,
    ));
    sword.weapon_attack_speed = 2.0;
    sword.weapon_is_auto = true;
    sword.create_world_entity(location, false, true, commands, meshes, materials);
}

fn create_handgun(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    location: Vec3,
) {
    let mut gun = InventoryItem::from((
        (0, 0, 0),
        vec![(0, 0, 0), (0, 0, 1), (0, -1, 0), (0, 0, 2)],
        Color::rgba(0.1, 0.1, 0.1, 1.0),
        RANGED_WEAPON,
        ItemTypeId::HandGun,
    ));
    gun.weapon_attack_speed = 2.0;
    gun.projectile_speed = 30.0;

    gun.create_world_entity(location, false, true, commands, meshes, materials);
}

fn create_supergun(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    location: Vec3,
) {
    let mut gun = InventoryItem::from((
        (0, 0, 0),
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (0, -1, 0),
            (0, 0, 2),
            (0, 0, 3),
            (0, 0, 4),
            (0, -1, 4),
            (0, 0, 5),
        ],
        Color::rgba(0.1, 0.1, 0.1, 1.0),
        RANGED_WEAPON,
        ItemTypeId::SuperGun,
    ));
    gun.weapon_attack_speed = 10.0;
    gun.projectile_speed = 30.0;

    gun.create_world_entity(location, false, true, commands, meshes, materials);
}

fn create_heart(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    location: Vec3,
) {
    let mut heart = InventoryItem::from((
        (0, 0, 0),
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (1, 0, 0),
            (-1, 0, -1),
            (1, 0, -1),
        ],
        Color::rgba(1.0, 0.1, 0.1, 1.0),
        NON_WEAPON,
        ItemTypeId::Heart,
    ));
    heart.hp_gain = 1;

    heart.create_world_entity(location, false, true, commands, meshes, materials);
}
pub fn spawn_random_item(
    mut luck: &mut Queue<i32>,
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let spawn_id = rng.gen_range(0..20);

    let position = Vec3::new(
        (random::<f32>() - 0.2) * ARENA_DIMENSIONS_METERS[0],
        0.5,
        (random::<f32>() - 0.2) * ARENA_DIMENSIONS_METERS[0],
    );

    match spawn_id {
        0 => {
            create_heart(commands, meshes, materials, position);
        }
        1..=19 => match luck.peek() {
            Ok(1) => {
                create_better_sword(commands, meshes, materials, position);
                let _ = luck.remove();
            }
            Ok(2) => {
                create_handgun(commands, meshes, materials, position);
                let _ = luck.remove();
            }
            Ok(3) => {
                create_supergun(commands, meshes, materials, position);
                let _ = luck.remove();
            }
            _ => {}
        },
        _ => {}
    }
}

fn monitor_spawns() {}
