use crate::inventory::{InventoryData, InventoryItem};
use crate::math::deg_to_rad;
use crate::voxel_renderer::{VoxelCoordinateFrame, GRID_DIMS};
use crate::GameState;
use bevy::input::mouse::MouseMotion;
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
                move_inventory_items,
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
                horizontal: deg_to_rad(180.0),
                vertical: deg_to_rad(-45.0),
                zoom_pos: 0.0,
            },
        }
    }
}

fn process_inputs(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut state: ResMut<InventoryControllerState>,
) {
    for motion_event in mouse_motion_events.iter() {
        state.unprocessed_delta = match state.unprocessed_delta {
            Some((x, y)) => Some((x + motion_event.delta.x, y + motion_event.delta.y)),
            None => Some((motion_event.delta.x, motion_event.delta.y)),
        };
    }

    state.rotate = mouse_buttons.pressed(MouseButton::Right);
    state.zoom = mouse_buttons.pressed(MouseButton::Left);
}

fn update_state(mut state: ResMut<InventoryControllerState>) {
    if let Some(unprocessed_delta) = state.unprocessed_delta {
        if state.rotate {
            let mouse_sensitivity = 0.002;

            state.orientation.horizontal += -unprocessed_delta.0 * mouse_sensitivity;
            state.orientation.vertical += -unprocessed_delta.1 * mouse_sensitivity;
        }

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

fn set_world_orientation(
    mut model_transform_query: Query<&mut Transform, With<VoxelCoordinateFrame>>,
    state: ResMut<InventoryControllerState>,
) {
    let mut world_transform = model_transform_query.single_mut();

    world_transform.translation.x = state.orientation.zoom_pos;
    world_transform.rotation = state.orientation.to_quat();
}

pub fn move_inventory_items(
    mut query: Query<&mut InventoryItem>,
    inv_coord_query: Query<&Transform, With<VoxelCoordinateFrame>>,
    camera_pos_query: Query<&Transform, With<Camera>>,
    k_input: Res<Input<KeyCode>>,
) {
    let inv_coord = inv_coord_query.single();
    let camera_coord = camera_pos_query.single();
    let _direction = (inv_coord.translation - camera_coord.translation).normalize();

    // println!("Test");
    // dbg!(direction);
    // dbg!(x_axis);
    // dbg!(y_axis);
    // dbg!(z_axis);
    for mut item in &mut query {
        if k_input.just_pressed(KeyCode::H) {
            item.translate(IVec3 { x: 1, y: 0, z: 0 })
        } else if k_input.just_pressed(KeyCode::L) {
            item.translate(IVec3 { x: -1, y: 0, z: 0 })
        } else if k_input.just_pressed(KeyCode::J) {
            item.translate(IVec3 { x: 0, y: 1, z: 0 })
        } else if k_input.just_pressed(KeyCode::K) {
            item.translate(IVec3 { x: 0, y: -1, z: 0 })
        }
    }
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
    mut game_state: ResMut<State<GameState>>,
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
