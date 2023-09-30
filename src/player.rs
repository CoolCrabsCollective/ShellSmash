use bevy::input::keyboard::KeyboardInput;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (process_inputs, player_movement));
        app.insert_resource(PlayerControllerState::new());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Collider::capsule_y(0.3, 0.25))
        .insert(SceneBundle {
            scene: asset_server.load("player.glb#Scene0"),
            ..default()
        })
        .insert(KinematicCharacterController {
            // The character offset is set to 0.01.
            offset: CharacterLength::Absolute(0.01),
            up: Vec3::Y,
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 1.0, 0.0)));
}

#[derive(Resource)]
struct PlayerControllerState {
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    velocity: Vec3,
}

impl PlayerControllerState {
    fn new() -> Self {
        Self {
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            velocity: vec3(0.0, 0.0, 0.0),
        }
    }
}

fn process_inputs(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut state: ResMut<PlayerControllerState>,
) {
    for event in keyboard_input_events.iter() {
        match event.key_code {
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
            _ => {}
        }
    }
}

fn player_movement(
    mut controllers: Query<&mut KinematicCharacterController>,
    time: Res<Time>,
    mut state: ResMut<PlayerControllerState>,
) {
    state.velocity.y -= 9.81 * time.delta_seconds();
    state.velocity.x /= 1.5;
    state.velocity.z /= 1.5;
    if state.is_forward_pressed {
        state.velocity.z = -6.0;
    }

    if state.is_backward_pressed {
        state.velocity.z = 6.0;
    }

    if state.is_left_pressed {
        state.velocity.x = -6.0;
    }

    if state.is_right_pressed {
        state.velocity.x = 6.0;
    }

    for mut controller in controllers.iter_mut() {
        controller.translation = Some(state.velocity * time.delta_seconds());
    }
}
