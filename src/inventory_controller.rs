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

    orientation: ControlledOrientation,
}

impl InventoryControllerState {
    pub fn new() -> Self {
        Self {
            unprocessed_delta: None,

            orientation: ControlledOrientation {
                horizontal: 0.0,
                vertical: deg_to_rad(-45.0),
            },
        }
    }
}

fn process_inputs(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut state: ResMut<InventoryControllerState>,
) {
    for event in mouse_motion_events.iter() {
        state.unprocessed_delta = match state.unprocessed_delta {
            Some((x, y)) => Some((x + event.delta.x, y + event.delta.y)),
            None => Some((event.delta.x, event.delta.y)),
        };
    }
}

fn update_state(mut state: ResMut<InventoryControllerState>) {
    if let Some(unprocessed_delta) = state.unprocessed_delta {
        let mouse_sensitivity = 0.002;

        state.orientation.horizontal += -unprocessed_delta.0 as f32 * mouse_sensitivity;
        state.orientation.vertical += -unprocessed_delta.1 as f32 * mouse_sensitivity;

        println!(
            "Horizontal: {:?} ({:?} rad)",
            rad_to_deg(state.orientation.horizontal),
            state.orientation.horizontal
        );

        println!(
            "Vertical: {:?} ({:?} rad)",
            rad_to_deg(state.orientation.vertical),
            state.orientation.vertical
        );
    }
    state.unprocessed_delta = None;
}

fn set_world_orientation(
    mut model_transform_query: Query<&mut Transform, With<Handle<Mesh>>>,
    state: Res<InventoryControllerState>,
) {
    let mut world_transform = model_transform_query.single_mut();

    world_transform.rotation = state.orientation.to_quat();
}

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}
