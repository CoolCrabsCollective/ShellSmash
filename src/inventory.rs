use crate::debug_camera_controller::DebugCameraControllerPlugin;
use crate::inventory_controller::InventoryControllerPlugin;
use bevy::log;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;

use crate::voxel_renderer::{VoxelCoordinateFrame, VoxelRendererPlugin, GRID_DIMS};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_plugins((WireframePlugin, VoxelRendererPlugin));
        app.add_systems(Update, (move_inventory_items, update_inventory_data));

        app.add_plugins(InventoryControllerPlugin);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    _asset_server: ResMut<AssetServer>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
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

#[derive(Component, Clone, Debug)]
pub struct InventoryItem {
    pub location: IVec3,          // world location
    pub local_points: Vec<IVec3>, // relative coordinate, center is the first point
    pub color: Color,
}

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

    pub fn _rotate_x(&mut self, ccw: bool) {
        let rot_angle = ((if ccw { 90 } else { -90 }) as f32).to_radians();

        let rot_mat = Mat3::from_rotation_x(rot_angle);
        for p in self.local_points.iter_mut() {
            let vec3 = Vec3::new(p.x as f32, p.y as f32, p.z as f32);
            let new_p: Vec3 = rot_mat.mul_vec3(vec3);
            p.x = new_p.x as i32;
            p.y = new_p.y as i32;
            p.z = new_p.z as i32;
        }
    }

    fn _get_center(&self) -> &IVec3 {
        self.local_points.first().unwrap()
    }
}

impl From<((i32, i32, i32), Vec<(i32, i32, i32)>, Color)> for InventoryItem {
    fn from(value: ((i32, i32, i32), Vec<(i32, i32, i32)>, Color)) -> Self {
        InventoryItem {
            location: value.0.into(),
            local_points: value.1.iter().map(|tup| (*tup).into()).collect(),
            color: value.2,
        }
    }
}

#[derive(Resource)]
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

pub fn update_inventory_data(query: Query<&InventoryItem>, mut inv: ResMut<InventoryData>) {
    let mut items: Vec<InventoryItem> = Vec::new();
    for p in query.iter() {
        items.push(p.clone())
    }
    inv.grid = InventoryData::grid_from_items(items, IVec3::from_array(GRID_DIMS))
}

#[derive(Debug)]
enum AxisSelect {
    X,
    Y,
    Z,
}

pub fn move_inventory_items(
    mut query: Query<&mut InventoryItem>,
    inv_coord_query: Query<&Transform, With<VoxelCoordinateFrame>>,
    camera_pos_query: Query<&Transform, With<Camera>>,
    k_input: Res<Input<KeyCode>>,
) {
    let camera_coord = camera_pos_query.get_single();
    if let Err(ref err) = camera_coord {
        log::error!(
            "Cancelling move_inventory_items since camera could not be initialized: {err:?}"
        );
        return;
    }
    let camera_coord = camera_coord.unwrap();

    let inv_coord = inv_coord_query.get_single();
    if let Err(ref err) = inv_coord {
        log::error!(
            "Cancelling move_inventory_items since inv coord could not be initialized: {err:?}"
        );
        return;
    }
    let inv_coord = inv_coord.unwrap();
    let direction = (inv_coord.translation - camera_coord.translation).normalize();
    let quat: Quat = inv_coord.rotation;
    let x_axis = quat
        .mul_vec3(Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        })
        .normalize();
    let y_axis = quat
        .mul_vec3(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        })
        .normalize();
    let z_axis = quat
        .mul_vec3(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        })
        .normalize();

    let mut principal_axis = x_axis;
    let mut axis_selected = AxisSelect::X;

    if principal_axis.dot(direction).abs() < y_axis.dot(direction).abs() {
        principal_axis = y_axis;
        axis_selected = AxisSelect::Y;
    }
    if principal_axis.dot(direction).abs() < z_axis.dot(direction).abs() {
        principal_axis = z_axis;
        axis_selected = AxisSelect::Z;
    }

    let sign: i32 = if principal_axis.dot(direction) < 0.0 {
        1
    } else {
        -1
    };
    let translation_up = IVec3 {
        x: 0,
        y: match axis_selected {
            AxisSelect::X => sign,
            AxisSelect::Z => sign,
            _ => 0,
        },
        z: match axis_selected {
            AxisSelect::Z => -sign,
            _ => 0,
        },
    };
    let translation_down = IVec3 {
        x: 0,
        y: match axis_selected {
            AxisSelect::X => -sign,
            AxisSelect::Z => -sign,
            _ => 0,
        },
        z: 0,
    };
    let translation_right = IVec3 {
        x: match axis_selected {
            AxisSelect::Y => sign,
            AxisSelect::Z => sign,
            _ => 0,
        },
        y: 0,
        z: match axis_selected {
            AxisSelect::X => -sign,
            _ => 0,
        },
    };
    let translation_left = IVec3 {
        x: match axis_selected {
            AxisSelect::Z => -sign,
            _ => 0,
        },
        y: 0,
        z: match axis_selected {
            AxisSelect::X => sign,
            _ => 0,
        },
    };

    // dbg!(axis_selected);
    for mut item in &mut query {
        if k_input.just_pressed(KeyCode::H) {
            item.translate(translation_left)
        } else if k_input.just_pressed(KeyCode::L) {
            item.translate(translation_right)
        } else if k_input.just_pressed(KeyCode::J) {
            item.translate(translation_down)
        } else if k_input.just_pressed(KeyCode::K) {
            item.translate(translation_up)
        }
    }
}
