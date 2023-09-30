use crate::voxel_renderer::VoxelCoordinateFrame;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct InventoryControllerPlugin;

impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                process_inputs,
                update_state,
                set_world_orientation,
                set_camera_pos,
            ),
        );
        app.insert_resource(InventoryControllerState::new());
    }
}

#[derive(Copy, Clone, Debug)]
struct ControlledOrientation {
    horizontal: f32,
    vertical: f32,
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

    orientation: ControlledOrientation,
    camera_pos: Vec3,
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
            },

            camera_pos: Vec3 {
                x: 15.0,
                y: 15.0,
                z: 5.0,
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
        let mouse_sensitivity = 0.002;

        let mouse_delta_0 = -unprocessed_delta.0 as f32 * mouse_sensitivity;
        let mouse_delta_1 = -unprocessed_delta.1 as f32 * mouse_sensitivity;

        if state.rotate {
            state.orientation.horizontal += mouse_delta_0;
            state.orientation.vertical += mouse_delta_1;
        }

        if state.zoom {
            state.camera_pos.x += mouse_delta_0;
            state.camera_pos.y += mouse_delta_1;
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
    state: Res<InventoryControllerState>,
) {
    let mut world_transform = model_transform_query.single_mut();

    world_transform.rotation = state.orientation.to_quat();
}

fn set_camera_pos(
    mut cam_transform_query: Query<&mut Transform, With<Camera>>,
    state: Res<InventoryControllerState>,
) {
    let mut cam_transform = cam_transform_query.single_mut();

    cam_transform.translation = state.camera_pos;
}

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}
