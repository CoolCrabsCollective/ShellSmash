use crate::game_state::GameState;
use crate::inventory::{InventoryData, InventoryItem};
use crate::math::deg_to_rad;
use crate::voxel_renderer::{VoxelCoordinateFrame, GRID_DIMS};
use bevy::{log, prelude::*};

pub struct InventoryControllerPlugin;

impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ManagingInventory), enter_inventory);
        app.add_systems(
            Update,
            update_camera_position.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            update_cube_rotation.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            move_inventory_items.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            update_state.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            update_inventory_data.run_if(in_state(GameState::ManagingInventory)),
        );
        app.insert_resource(InventoryControllerState::new());
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ControlledOrientation {
    horizontal: f32,
    vertical: f32,
    zoom_pos: f32,
}

impl ControlledOrientation {
    pub fn to_quat(self) -> Quat {
        Quat::from_euler(EulerRot::XYZ, 0.0, self.horizontal, 0.0)
            * Quat::from_euler(EulerRot::XYZ, self.vertical, 0.0, 0.0)
    }
}

// #[derive(Resource)]
// struct RotateAnime {
//     enabled: bool;
// }

#[derive(Resource)]
struct InventoryControllerState {
    unprocessed_delta: Option<(f32, f32)>,

    rotate: bool,
    zoom: bool,
    pub view_index: usize,
    pub orientation: ControlledOrientation,
}

impl InventoryControllerState {
    pub fn new() -> Self {
        Self {
            unprocessed_delta: None,

            rotate: false,
            zoom: false,
            view_index: 0,

            orientation: ControlledOrientation {
                horizontal: deg_to_rad(0.0),
                vertical: deg_to_rad(0.0),
                zoom_pos: 0.0,
            },
        }
    }
}

// fn process_inputs(
//     key_codes: Res<Input<KeyCode>>,
//     mut state: ResMut<InventoryControllerState>,
//     mut query: <(
//         Query<&mut Transform, With<VoxelCoordinateFrame>>,
//         Query<&mut Transform, With<Camera>>,
//     )>,
// ) {
//     let mut voxel_translation_p0 = param_set.p0();
//     let mut voxel_translation = voxel_translation_p0.single_mut();
//     let vox_trans = voxel_translation.translation;
//     let views: Vec<f32> = vec![0.0, 90.0, 180.0, 270.0];
//     voxel_translation.rotation = Quat::from_rotation_y(deg_to_rad(views[state.view_index]));
//     camera_translation.translation = vox_trans + Vec3::from((0.0, cam_distance, cam_distance));
//     let look_at_my_balls = camera_translation.looking_at(vox_trans, Vec3::Y);
//     camera_translation.rotation = look_at_my_balls.rotation;
// }

fn update_cube_rotation(
    key_codes: Res<Input<KeyCode>>,
    mut state: ResMut<InventoryControllerState>,
    mut query: Query<&mut Transform, With<VoxelCoordinateFrame>>,
) {
    let possible_rotations: Vec<f32> = vec![0.0, 90.0, 180.0, 270.0];
    if key_codes.just_pressed(KeyCode::Left) {
        state.view_index = if state.view_index == 0 {
            3
        } else {
            state.view_index - 1
        };
    } else if key_codes.just_pressed(KeyCode::Right) {
        state.view_index = (state.view_index + 1) % 4;
    }
    let mut vox = query.single_mut();
    vox.rotation = Quat::from_rotation_y(deg_to_rad(possible_rotations[state.view_index]));
}

fn update_camera_position(
    mut param_set: ParamSet<(
        Query<&Transform, With<VoxelCoordinateFrame>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let cam_distance = 8.0;
    let vox_trans = {
        let vox_trans_query = param_set.p0();
        vox_trans_query.single().translation
    };
    let mut camera_translation_query = param_set.p1();
    let mut camera_translation = camera_translation_query.single_mut();
    camera_translation.translation = vox_trans + Vec3::from((0.0, cam_distance, cam_distance));
    let look_at_my_balls = camera_translation.looking_at(vox_trans, Vec3::Y);
    camera_translation.rotation = look_at_my_balls.rotation;
}

fn update_state(mut state: ResMut<InventoryControllerState>) {
    if let Some(unprocessed_delta) = state.unprocessed_delta {
        if state.zoom {
            let mouse_sensitivity = 0.02;

            state.orientation.zoom_pos += unprocessed_delta.1 * mouse_sensitivity;
        }
    }
    state.unprocessed_delta = None;
    state.rotate = false;
    state.zoom = false;
}

pub fn update_inventory_data(query: Query<&InventoryItem>, mut inv: ResMut<InventoryData>) {
    let mut items: Vec<InventoryItem> = Vec::new();
    for p in query.iter() {
        items.push(p.clone())
    }
    inv.grid = InventoryData::grid_from_items(items, IVec3::from_array(GRID_DIMS))
}

fn get_initial_camera_transform() -> Transform {
    Transform::default()
        .with_translation(Vec3::new(500.0, 8.0, 8.0))
        .looking_at(Vec3::new(500.0, 0.0, 0.0), Vec3::Y)
}

fn enter_inventory(mut cam_transform_query: Query<&mut Transform, With<Camera>>) {
    (*cam_transform_query.single_mut()) = get_initial_camera_transform();
}

fn move_inventory_items(
    state: Res<InventoryControllerState>,
    key_codes: Res<Input<KeyCode>>,
    mut query_items: Query<&mut InventoryItem>,
) {
    let trans: Vec<IVec3> = vec![
        IVec3::from((0, 0, -1)),
        IVec3::from((-1, 0, 0)),
        IVec3::from((0, 0, 1)),
        IVec3::from((1, 0, 0)),
    ];
    let view_index = state.view_index;
    dbg!(view_index);
    if key_codes.just_pressed(KeyCode::W) {
        for mut item in query_items.iter_mut() {
            item.translate(trans[(4 - view_index) % 4]);
        }
    } else if key_codes.just_pressed(KeyCode::A) {
        for mut item in query_items.iter_mut() {
            item.translate(
                trans[if 1 <= view_index {
                    (5 - view_index) % 4
                } else {
                    1 - view_index
                }],
            );
        }
    } else if key_codes.just_pressed(KeyCode::S) {
        for mut item in query_items.iter_mut() {
            item.translate(
                trans[if 2 <= view_index {
                    (6 - view_index) % 4
                } else {
                    2 - view_index
                }],
            );
        }
    } else if key_codes.just_pressed(KeyCode::D) {
        for mut item in query_items.iter_mut() {
            item.translate(
                trans[if 3 <= view_index {
                    (7 - view_index) % 4
                } else {
                    3 - view_index
                }],
            );
        }
    }
}
