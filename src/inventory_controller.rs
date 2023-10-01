use crate::inventory::{InventoryData, InventoryItem};
use crate::math::deg_to_rad;
use crate::voxel_renderer::{VoxelCoordinateFrame, GRID_DIMS};
use crate::GameState;
use bevy::prelude::*;

pub struct InventoryControllerPlugin;

impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enter_inventory);
        app.add_systems(
            Update,
            (
                process_inputs,
                update_state,
                set_world_orientation,
                update_inventory_data,
            ),
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

    pub orientation: ControlledOrientation,
}

impl InventoryControllerState {
    pub fn new() -> Self {
        Self {
            unprocessed_delta: None,

            rotate: false,
            zoom: false,

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
    mouse_buttons: Res<Input<MouseButton>>,
    mut state: ResMut<InventoryControllerState>,
) {
    if key_codes.just_pressed(KeyCode::W) {
        state.orientation.vertical += deg_to_rad(90.0);
    } else if key_codes.just_pressed(KeyCode::S) {
        state.orientation.vertical += deg_to_rad(-90.0);
    } else if key_codes.just_pressed(KeyCode::A) {
        state.orientation.horizontal += deg_to_rad(-90.0);
    } else if key_codes.just_pressed(KeyCode::D) {
        state.orientation.horizontal += deg_to_rad(90.0);
    }

    // for motion_event in mouse_motion_events.iter() {
    //     state.unprocessed_delta = match state.unprocessed_delta {
    //         Some((x, y)) => Some((x + motion_event.delta.x, y + motion_event.delta.y)),
    //         None => Some((motion_event.delta.x, motion_event.delta.y)),
    //     };
    // }
    //
    // state.rotate = mouse_buttons.pressed(MouseButton::Right);
    state.zoom = mouse_buttons.pressed(MouseButton::Left);
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

#[allow(clippy::type_complexity)]
fn set_world_orientation(
    mut param_set: ParamSet<(
        Query<&mut Transform, With<VoxelCoordinateFrame>>,
        Query<&Transform, With<Camera>>,
    )>,
    state: ResMut<InventoryControllerState>,
) {
    let base_camera_translation = {
        let camera_transform_query = param_set.p1();
        let camera_transform = camera_transform_query.single();
        camera_transform.translation + camera_transform.forward() * 2.0 * GRID_DIMS[0] as f32
    };

    let mut model_transform_query = param_set.p0();
    let world_transform = model_transform_query.get_single_mut();
    if let Err(ref _err) = world_transform {
        return;
    }
    let mut world_transform = world_transform.unwrap();

    world_transform.translation = base_camera_translation;
    world_transform.translation.x += state.orientation.zoom_pos;
    world_transform.rotation = state.orientation.to_quat();
}

pub fn update_inventory_data(query: Query<&InventoryItem>, mut inv: ResMut<InventoryData>) {
    let mut items: Vec<InventoryItem> = Vec::new();
    for p in query.iter() {
        items.push(p.clone())
    }
    inv.grid = InventoryData::grid_from_items(items, IVec3::from_array(GRID_DIMS))
}

fn enter_inventory(
    mut cam_transform_query: Query<&mut Transform, With<Camera>>,
    game_state: ResMut<State<GameState>>,
) {
    if *game_state.get() != GameState::Inventory {
        return;
    }

    let mut cam_transform = cam_transform_query.single_mut();

    cam_transform.translation = Vec3 {
        x: -15.0,
        y: 5.0,
        z: 0.0,
    };
}
