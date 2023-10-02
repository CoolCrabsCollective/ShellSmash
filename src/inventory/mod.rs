use bevy::math::vec3;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy::utils::HashSet;

use crate::asset_loader::GameAssets;
use crate::config::{DEFAULT_BAG_LOCATION, INVENTORY_GRID_DIMENSIONS};
use crate::game_state::GameState;
use crate::inventory::controller::InventoryControllerPlugin;
use crate::inventory::controller::ItemDirection;
use crate::inventory::data_manager::InventoryDataPlugin;
use crate::inventory::gizmo::Gizmo;
use crate::inventory::grid::GridDisplayPlugin;
use crate::inventory::validation::InventoryValidationPlugin;
use crate::math::deg_to_rad;

mod controller;
mod data_manager;
mod gizmo;
mod grid;
mod selection;
mod ui;
mod validation;
mod weapon_selector;

use crate::inventory::selection::{SelectedItem, SelectionPlugin};
use crate::inventory::ui::InventoryUIPlugin;
pub use weapon_selector::WeaponSelectorPlugin;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ManagingInventory), setup);
        app.add_systems(
            Update,
            update_packed_items.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(Update, make_sure_no_weapon_duplicates);
        app.add_plugins((
            InventoryControllerPlugin,
            InventoryDataPlugin,
            GridDisplayPlugin,
            InventoryValidationPlugin,
            InventoryUIPlugin,
            SelectionPlugin,
        ))
        .insert_resource(Inventory {
            content: Vec::new(),
        })
        .insert_resource(InventoryData { grid: Vec::new() });
    }
}

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut inventory: ResMut<Inventory>,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selection: ResMut<SelectedItem>,
) {
    let mut up_transform =
        Transform::from_translation(DEFAULT_BAG_LOCATION + Vec3::from((0.0, 0.0, 0.0)));
    up_transform.rotation =
        Quat::from_euler(EulerRot::XYZ, deg_to_rad(-90.0), deg_to_rad(0.0), 0.0);
    let mut down_transform =
        Transform::from_translation(DEFAULT_BAG_LOCATION + Vec3::from((0.0, 0.0, 0.0)));
    down_transform.rotation =
        Quat::from_euler(EulerRot::XYZ, deg_to_rad(90.0), deg_to_rad(0.0), 0.0);
    let mut left_transform =
        Transform::from_translation(DEFAULT_BAG_LOCATION + Vec3::from((0.0, 0.0, 0.0)));
    left_transform.rotation = Quat::from_euler(
        EulerRot::XYZ,
        deg_to_rad(0.0),
        deg_to_rad(-90.0),
        deg_to_rad(90.0),
    );
    let mut right_transform =
        Transform::from_translation(DEFAULT_BAG_LOCATION + Vec3::from((0.0, 0.0, 0.0)));
    right_transform.rotation = Quat::from_euler(
        EulerRot::XYZ,
        deg_to_rad(0.0),
        deg_to_rad(90.0),
        deg_to_rad(90.0),
    );

    let mut backwards_transform =
        Transform::from_translation(DEFAULT_BAG_LOCATION + Vec3::from((0.0, 0.0, 0.0)));
    backwards_transform.rotation = Quat::from_euler(
        EulerRot::XYZ,
        deg_to_rad(0.0),
        deg_to_rad(0.0),
        deg_to_rad(0.0),
    );
    let mut forward_transform =
        Transform::from_translation(DEFAULT_BAG_LOCATION + Vec3::from((0.0, 0.0, 0.0)));
    forward_transform.rotation = Quat::from_euler(
        EulerRot::XYZ,
        deg_to_rad(0.0),
        deg_to_rad(180.0),
        deg_to_rad(0.0),
    );

    commands
        .spawn(Gizmo {
            relative: up_transform,
            item_dir: ItemDirection::UP,
        })
        .insert(PbrBundle {
            mesh: game_assets.arrow_straight().mesh_handle,
            material: game_assets.arrow_straight().material_handle,
            ..default()
        })
        .insert(NotShadowReceiver);
    commands
        .spawn(Gizmo {
            relative: down_transform,
            item_dir: ItemDirection::DOWN,
        })
        .insert(PbrBundle {
            mesh: game_assets.arrow_straight().mesh_handle,
            material: game_assets.arrow_straight().material_handle,
            ..default()
        })
        .insert(NotShadowReceiver);
    commands
        .spawn(Gizmo {
            relative: left_transform,
            item_dir: ItemDirection::LEFT,
        })
        .insert(PbrBundle {
            mesh: game_assets.arrow_straight().mesh_handle,
            material: game_assets.arrow_straight().material_handle,
            ..default()
        })
        .insert(NotShadowReceiver);
    commands
        .spawn(Gizmo {
            relative: right_transform,
            item_dir: ItemDirection::RIGHT,
        })
        .insert(PbrBundle {
            mesh: game_assets.arrow_straight().mesh_handle,
            material: game_assets.arrow_straight().material_handle,
            ..default()
        })
        .insert(NotShadowReceiver);
    commands
        .spawn(Gizmo {
            relative: forward_transform,
            item_dir: ItemDirection::FORWARD,
        })
        .insert(PbrBundle {
            mesh: game_assets.arrow_straight().mesh_handle,
            material: game_assets.arrow_straight().material_handle,
            ..default()
        })
        .insert(NotShadowReceiver);
    commands
        .spawn(Gizmo {
            relative: backwards_transform,
            item_dir: ItemDirection::BACKWARDS,
        })
        .insert(PbrBundle {
            mesh: game_assets.arrow_straight().mesh_handle,
            material: game_assets.arrow_straight().material_handle,
            ..default()
        })
        .insert(NotShadowReceiver);

    // Render current inventory data

    let mut id = None;

    for item in &inventory.content {
        id = Some(
            commands
                .spawn(PackedInventoryItem { data: item.clone() })
                .insert(PbrBundle {
                    mesh: meshes.add(item.generate_mesh()),
                    material: materials.add(item.color.clone().into()),
                    transform: Transform::from_translation(
                        DEFAULT_BAG_LOCATION + item.location.as_vec3()
                            - vec3(
                                (INVENTORY_GRID_DIMENSIONS[0] / 2) as f32,
                                (INVENTORY_GRID_DIMENSIONS[1] / 2) as f32,
                                (INVENTORY_GRID_DIMENSIONS[2] / 2) as f32,
                            ),
                    ),
                    ..default()
                })
                .id(),
        );
    }

    selection.selected_entity = id;
}

// updates visual positions of items in packed inventory UI
fn update_packed_items(
    mut commands: Commands,
    mut query: Query<(
        &mut Transform,
        &mut Handle<Mesh>,
        &mut PackedInventoryItem,
        Entity,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for mut item in query.iter_mut() {
        item.0.translation = DEFAULT_BAG_LOCATION + item.2.data.location.as_vec3()
            - vec3(
                (INVENTORY_GRID_DIMENSIONS[0] / 2) as f32,
                (INVENTORY_GRID_DIMENSIONS[1] / 2) as f32,
                (INVENTORY_GRID_DIMENSIONS[2] / 2) as f32,
            );

        if !item.2.data.changed {
            continue;
        }

        commands.entity(item.3).remove::<Handle<Mesh>>();
        commands
            .entity(item.3)
            .insert(meshes.add(item.2.data.generate_mesh()));
        item.2.data.changed = false;
    }
}

#[derive(Component, Clone, Debug)]
pub struct PackedInventoryItem {
    pub data: InventoryItem,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    MELEE_WEAPON,
    RANGED_WEAPON,
    NON_WEAPON,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ItemTypeId {
    Boomerang,
    DavidGun,
    WillSword,
    MidSword,
    AlexSword,
    Heart,
}

#[derive(Clone, Debug, Component, PartialEq)]
pub struct InventoryItem {
    pub location: IVec3, // grid location
    pub original_points: Vec<IVec3>,
    pub local_points: Vec<IVec3>, // relative coordinate, center is the first point
    pub changed: bool,
    pub color: Color,

    pub hp_gain: i32,            // how much HP this item gives you for having it
    pub attack_damage_gain: i32, // how much attack damage this item gives you for having it
    pub attack_speed_gain: f32,  // how much attack speed this item gives you for having it

    pub weapon_damage: i32, // how much base attack damage this item does when used as a weapon
    pub weapon_attack_speed: f32, // how much base attack speed this item has when used as a weapon
    pub weapon_is_auto: bool, // whether holding click auto attacks for this weapon

    pub projectile_speed: f32, // how fast the ranged weapon's 'bullets' travel

    pub item_type: ItemType,
    pub item_type_id: ItemTypeId,
}

// inventory of what the user owns currently
// query the resource to get it
#[derive(Resource)]
pub struct Inventory {
    pub content: Vec<InventoryItem>,
}

#[derive(Debug)]
pub struct InventoryItemInfo {
    pub color: Color,
}

impl InventoryItem {
    pub fn intersects(&self, other_location: IVec3) -> bool {
        let relative_location: IVec3 = self.location - other_location;
        for point in &self.local_points {
            if *point == relative_location {
                return true;
            }
        }
        false
    }

    pub fn translate(&mut self, translation: IVec3) {
        self.location += translation;
    }

    pub fn rotate(&mut self, ccw: bool) {
        let rot_angle = ((if ccw { 90 } else { -90 }) as f32).to_radians();

        let rot_mat = Mat3::from_rotation_y(rot_angle);
        for p in self.local_points.iter_mut() {
            let vec3 = Vec3::new(p.x as f32, p.y as f32, p.z as f32);
            let new_p: Vec3 = rot_mat.mul_vec3(vec3);
            p.x = new_p.x as i32;
            p.y = new_p.y as i32;
            p.z = new_p.z as i32;
        }
        self.changed = true;
    }

    #[allow(dead_code)]
    fn get_center(&self) -> &IVec3 {
        self.local_points.first().unwrap()
    }
}

impl
    From<(
        (i32, i32, i32),
        Vec<(i32, i32, i32)>,
        Color,
        ItemType,
        ItemTypeId,
    )> for InventoryItem
{
    fn from(
        value: (
            (i32, i32, i32),
            Vec<(i32, i32, i32)>,
            Color,
            ItemType,
            ItemTypeId,
        ),
    ) -> Self {
        InventoryItem {
            location: value.0.into(),
            local_points: value.1.iter().map(|tup| (*tup).into()).collect(),
            original_points: value.1.iter().map(|tup| (*tup).into()).collect(),
            color: value.2,
            hp_gain: 0,
            attack_damage_gain: 0,
            attack_speed_gain: 0.0,
            weapon_damage: 1,
            weapon_is_auto: false,
            weapon_attack_speed: 1.0,
            item_type: value.3,
            projectile_speed: 1.0,
            changed: false,
            item_type_id: value.4,
        }
    }
}

#[derive(Resource, Debug)]
pub struct InventoryData {
    pub grid: Vec<Vec<Vec<Option<InventoryItemInfo>>>>,
}

impl InventoryData {
    pub fn grid_from_items(
        items: Vec<InventoryItem>,
        grid_size: IVec3,
    ) -> Vec<Vec<Vec<Option<InventoryItemInfo>>>> {
        let mut item_grid: Vec<Vec<Vec<Option<InventoryItemInfo>>>> = Vec::new();
        for x in 0..grid_size.x {
            let mut rows: Vec<Vec<Option<InventoryItemInfo>>> = Vec::new();
            for y in 0..grid_size.y {
                let mut cols: Vec<Option<InventoryItemInfo>> = Vec::new();
                for z in 0..grid_size.z {
                    let mut item_found = false;
                    for i in &items {
                        if i.intersects(IVec3 { x, y, z }) {
                            item_found = true;
                            cols.push(Some(InventoryItemInfo { color: i.color }));
                            break;
                        }
                    }

                    if !item_found {
                        cols.push(None);
                    }
                }
                rows.push(cols);
            }
            item_grid.push(rows);
        }
        item_grid
    }
}

fn make_sure_no_weapon_duplicates(inventory: Res<Inventory>) {
    let mut type_id_set: HashSet<ItemTypeId> = HashSet::new();

    for item in &inventory.content {
        if item.item_type == ItemType::NON_WEAPON {
            continue;
        }

        assert!(
            !type_id_set.contains(&item.item_type_id),
            "Not allowed two non-weapons with the same item type id in the inventory, fuck you"
        );

        type_id_set.insert(item.item_type_id);
    }
}
