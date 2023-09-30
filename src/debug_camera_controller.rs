use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseMotion;
use bevy::math::Vec3;
use bevy::prelude::*;

use crate::math::deg_to_rad;

pub struct DebugCameraControllerPlugin;

impl Plugin for DebugCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (process_inputs, update_state, set_camera));
        app.insert_resource(CameraControllerState::new());
    }
}

#[derive(Copy, Clone, Debug)]
struct ControlledViewDirection {
    horizontal: f32,
    vertical: f32,
}

impl ControlledViewDirection {
    fn to_quat(self) -> Quat {
        Quat::from_euler(EulerRot::XYZ, 0.0, self.horizontal, 0.0)
            * Quat::from_euler(EulerRot::XYZ, self.vertical, 0.0, 0.0)
    }

    fn to_vector(self) -> Vec3 {
        let horizontal_scale = self.vertical.cos();
        Vec3::new(
            (self.horizontal + std::f32::consts::PI).sin() * horizontal_scale,
            self.vertical.sin(),
            (self.horizontal + std::f32::consts::PI).cos() * horizontal_scale,
        )
        .normalize()
    }
}

#[derive(Resource)]
struct CameraControllerState {
    unprocessed_delta: Option<(f32, f32)>,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,

    view_direction: ControlledViewDirection,
    position: Vec3,
    speed: f32,

    enabled: bool,
}

impl CameraControllerState {
    fn new() -> Self {
        Self {
            unprocessed_delta: None,

            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,

            view_direction: ControlledViewDirection {
                horizontal: 0.0,
                vertical: deg_to_rad(-45.0),
            },
            position: Vec3::new(0.0, 3.0, 3.0),
            speed: 5.0,
            enabled: false,
        }
    }
}

fn process_inputs(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut state: ResMut<CameraControllerState>,
) {
    for event in mouse_motion_events.iter() {
        state.unprocessed_delta = match state.unprocessed_delta {
            Some((x, y)) => Some((x + event.delta.x, y + event.delta.y)),
            None => Some((event.delta.x, event.delta.y)),
        };
    }

    for event in keyboard_input_events.iter() {
        match event.key_code {
            Some(KeyCode::F5) => {
                state.enabled = true;
            }
            Some(KeyCode::W) => {
                state.is_forward_pressed = event.state.is_pressed();
            }
            Some(KeyCode::S) => {
                state.is_backward_pressed = event.state.is_pressed();
            }
            Some(KeyCode::A) => {
                state.is_left_pressed = event.state.is_pressed();
            }
            Some(KeyCode::D) => {
                state.is_right_pressed = event.state.is_pressed();
            }
            Some(KeyCode::E) => {
                state.is_up_pressed = event.state.is_pressed();
            }
            Some(KeyCode::Q) => {
                state.is_down_pressed = event.state.is_pressed();
            }
            _ => {}
        }
    }
}

fn update_state(mut state: ResMut<CameraControllerState>, time: Res<Time>) {
    if !state.enabled {
        return;
    }

    if let Some(unprocessed_delta) = state.unprocessed_delta {
        let mouse_sensitivity = 0.002;

        state.view_direction.horizontal += -unprocessed_delta.0 * mouse_sensitivity;
        state.view_direction.vertical = (state.view_direction.vertical
            + (-unprocessed_delta.1 * mouse_sensitivity))
            .clamp(deg_to_rad(-90.0), deg_to_rad(90.0));
    }
    state.unprocessed_delta = None;

    let movement_vector = {
        let forward_direction = state.view_direction.to_vector();
        let up_direction = Vec3::new(0.0, 1.0, 0.0);
        let right_direction = forward_direction.cross(up_direction);

        let mut movement_vector: Option<Vec3> = None;
        let mut add_movement = |movement: Vec3| {
            movement_vector = match movement_vector {
                Some(res) => Some(res + movement),
                None => Some(movement),
            }
        };

        if state.is_forward_pressed {
            add_movement(forward_direction);
        } else if state.is_backward_pressed {
            add_movement(-forward_direction);
        }

        if state.is_right_pressed {
            add_movement(right_direction);
        } else if state.is_left_pressed {
            add_movement(-right_direction);
        }

        if state.is_up_pressed {
            add_movement(up_direction);
        } else if state.is_down_pressed {
            add_movement(-up_direction);
        }

        movement_vector
            .map(|movement_vector| {
                movement_vector.normalize() * state.speed * time.delta().as_secs_f32()
            })
            .unwrap_or(Vec3::new(0.0, 0.0, 0.0))
    };

    state.position += movement_vector;
}

fn set_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    mut state: ResMut<CameraControllerState>,
) {
    let mut camera_transform = camera_transform_query.single_mut();

    if !state.enabled {
        state.position = camera_transform.translation;
        let res = camera_transform.rotation.to_euler(EulerRot::XYZ);
        state.view_direction.vertical = res.0;
        state.view_direction.horizontal = res.1;
        return;
    }

    camera_transform.translation = state.position;
    camera_transform.rotation = state.view_direction.to_quat();
}
