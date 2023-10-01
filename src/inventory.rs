use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;

use crate::config::INVENTORY_GRID_DIMENSIONS;
use crate::voxel_renderer::VoxelRendererPlugin;
use crate::{game_state::GameState, inventory_controller::InventoryControllerPlugin};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ManagingInventory), setup);
        app.add_plugins((
            WireframePlugin,
            VoxelRendererPlugin,
            InventoryControllerPlugin,
        ))
        .insert_resource(Inventory {
            content: Vec::new(),
        })
        .insert_resource(InventoryData { grid: Vec::new() });
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    let boomerang = InventoryItem::from((
        (3, 0, 0),
        vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
        Color::rgba(1.0, 1.0, 1.0, 1.0),
    ));
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
    ));
    let heart = InventoryItem::from((
        (4, 1, 1),
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

    commands.spawn(VoxelBullcrap { data: sword });
}

#[derive(Component, Clone, Debug)]
pub struct VoxelBullcrap {
    pub data: InventoryItem,
}

#[derive(Clone, Debug)]
pub struct InventoryItem {
    pub location: IVec3, // grid location
    pub original_points: Vec<IVec3>,
    pub local_points: Vec<IVec3>, // relative coordinate, center is the first point
    pub color: Color,

    pub hp_gain: i32,            // how much HP this item gives you for having it
    pub attack_damage_gain: i32, // how much attack damage this item gives you for having it
    pub attack_speed_gain: f32,  // how much attack speed this item gives you for having it

    pub weapon_damage: i32, // how much base attack damage this item does when used as a weapon
    pub weapon_is_auto: bool, // whether holding click auto attacks for this weapon
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
    }

    #[allow(dead_code)]
    fn get_center(&self) -> &IVec3 {
        self.local_points.first().unwrap()
    }
}

impl From<((i32, i32, i32), Vec<(i32, i32, i32)>, Color)> for InventoryItem {
    fn from(value: ((i32, i32, i32), Vec<(i32, i32, i32)>, Color)) -> Self {
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
