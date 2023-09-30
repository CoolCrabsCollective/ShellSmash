use crate::math::deg_to_rad;
use crate::voxel_renderer::VoxelCoordinateFrame;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct InventoryControllerPlugin;

impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (process_inputs, update_state, set_world_orientation),
        );
        app.insert_resource(InventoryControllerState::new());
    }
}

#[derive(Copy, Clone, Debug)]
struct ControlledOrientation {
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

    orientation: ControlledOrientation,
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

            state.orientation.horizontal += -unprocessed_delta.0 as f32 * mouse_sensitivity;
            state.orientation.vertical += -unprocessed_delta.1 as f32 * mouse_sensitivity;
        }

        if state.zoom {
            let mouse_sensitivity = 0.02;

            state.orientation.zoom_pos += unprocessed_delta.1 as f32 * mouse_sensitivity;
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

    world_transform.translation.x = state.orientation.zoom_pos;
    world_transform.rotation = state.orientation.to_quat();
}
