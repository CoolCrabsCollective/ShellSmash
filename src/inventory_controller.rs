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
            process_inputs.run_if(in_state(GameState::ManagingInventory)),
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

fn process_inputs(
    // mut mouse_motion_events: EventReader<MouseMotion>,
    key_codes: Res<Input<KeyCode>>,
    mut state: ResMut<InventoryControllerState>,
    mut param_set: ParamSet<(
        Query<&Transform, With<VoxelCoordinateFrame>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let cam_distance = 8.0;

    let voxel_translation_p0 = param_set.p0();
    let voxel_translation = voxel_translation_p0.single();
    let vox_trans = voxel_translation.translation;
    let views: Vec<Vec3> = vec![
        Vec3::from((cam_distance, cam_distance, 0.0)),
        Vec3::from((0.0, cam_distance, cam_distance)),
        Vec3::from((-cam_distance, cam_distance, 0.0)),
        Vec3::from((0.0, cam_distance, -cam_distance)),
    ];

    if key_codes.just_pressed(KeyCode::Left) {
        let mut camera_translation_query = param_set.p1();
        let mut camera_translation = camera_translation_query.single_mut();
        camera_translation.translation = vox_trans + views[state.view_index];
        state.view_index = (state.view_index + 1) % 4;
        let look_at = camera_translation.looking_at(vox_trans, Vec3::Y);
        camera_translation.rotation = look_at.rotation;
    } else if key_codes.just_pressed(KeyCode::Right) {
        let mut camera_translation_query = param_set.p1();
        let mut camera_translation = camera_translation_query.single_mut();
        camera_translation.translation = vox_trans + views[state.view_index];
        state.view_index = if state.view_index == 0 {
            3
        } else {
            state.view_index - 1
        };
        let look_at_my_balls = camera_translation.looking_at(vox_trans, Vec3::Y);
        camera_translation.rotation = look_at_my_balls.rotation;
    }
}

fn update_state(mut state: ResMut<InventoryControllerState>) {
    if let Some(unprocessed_delta) = state.unprocessed_delta {
        // if state.rotate {
        //     let mouse_sensitivity = 0.002;
        //
        //     state.orientation.horizontal += -unprocessed_delta.0 * mouse_sensitivity;
        //     state.orientation.vertical += -unprocessed_delta.1 * mouse_sensitivity;
        // }

        if state.zoom {
            let mouse_sensitivity = 0.02;

            state.orientation.zoom_pos += unprocessed_delta.1 * mouse_sensitivity;
        }

        // println!(
        //     "Horizontal: {:?} ({:?} rad)",
        //     rad_to_deg(state.orientation.horizontal),
        //     state.orientation.horizontal
        // );

        // println!(
        //     "Vertical: {:?} ({:?} rad)",
        //     rad_to_deg(state.orientation.vertical),
        //     state.orientation.vertical
        // );
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
    Transform::default().with_translation(Vec3::new(500.0, 0.0, 0.0))
}

fn enter_inventory(mut cam_transform_query: Query<&mut Transform, With<Camera>>) {
    (*cam_transform_query.single_mut()) = get_initial_camera_transform();
}
