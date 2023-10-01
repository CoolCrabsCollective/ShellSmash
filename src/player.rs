use bevy::input::keyboard::KeyboardInput;
use bevy::math::vec3;
use bevy::window::PrimaryWindow;
use bevy::{log, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::enemy::Enemy;
use crate::game_state::GameState;
use crate::world_item::WeaponHolder;

pub const PLAYER_HEIGHT: f32 = 0.6;
pub const PLAYER_WIDTH: f32 = 0.5;

pub struct PlayerPlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    Fighting,
    Dying,
}

#[derive(Component)]
pub struct PlayerControllerState {
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    velocity: Vec3,
}

#[derive(Event)]
pub struct PlayerHitEvent(Entity);

#[derive(Resource)]
struct DeathTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(OnEnter(GameState::FightingInArena), set_player_active);
        app.add_systems(
            Update,
            process_inputs.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_systems(
            Update,
            player_movement.run_if(
                in_state(GameState::FightingInArena).and_then(in_state(PlayerState::Fighting)),
            ),
        );
        app.add_systems(
            Update,
            detect_player_hit.run_if(
                in_state(GameState::FightingInArena).and_then(in_state(PlayerState::Fighting)),
            ),
        );
        app.add_systems(
            Update,
            handle_player_hit.run_if(
                in_state(GameState::FightingInArena).and_then(in_state(PlayerState::Fighting)),
            ),
        );
        app.add_systems(
            Update,
            tick_death_timer.run_if(
                in_state(GameState::FightingInArena).and_then(in_state(PlayerState::Dying)),
            ),
        );
        app.add_state::<PlayerState>();
        app.add_event::<PlayerHitEvent>();
        app.insert_resource(DeathTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    }
}

fn spawn_player(
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
        .insert(PlayerControllerState::new())
        .insert(WeaponHolder {
            current_weapon: None,
        })
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 1.0, 0.0)));
}

fn set_player_active(mut next_state: ResMut<NextState<PlayerState>>) {
    next_state.set(PlayerState::Fighting);
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
    mut state: Query<&mut PlayerControllerState>,
) {
    let mut state = state.single_mut();
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
    mut controllers: Query<&mut KinematicCharacterController, With<PlayerControllerState>>,
    time: Res<Time>,
    mut state: Query<&mut PlayerControllerState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut transform: Query<&mut Transform, With<PlayerControllerState>>,
) {
    let mut state = state.single_mut();
    let mut transform = transform.single_mut();

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

    controllers.single_mut().translation = Some(state.velocity * time.delta_seconds());

    let (camera, camera_transform) = camera_q.single();

    if let Some(position) = windows.single().cursor_position() {
        let ray: Ray = camera
            .viewport_to_world(camera_transform, position)
            .unwrap();
        if let Some(distance) =
            ray.intersect_plane(vec3(0.0, transform.translation.y, 0.0), vec3(0.0, 1.0, 0.0))
        {
            let pos = ray.get_point(distance);
            transform.look_at(pos, Vec3::Y);
        }
    }
}

fn detect_player_hit(
    player_controller_output_query: Query<
        &KinematicCharacterControllerOutput,
        With<PlayerControllerState>,
    >,
    enemy_entity_query: Query<Entity, With<Enemy>>,
    mut player_hit_event_writer: EventWriter<PlayerHitEvent>,
) {
    let player_controller_output = player_controller_output_query.get_single();
    if let Err(_error_ignored) = player_controller_output {
        return;
    }
    let player_controller_output = player_controller_output.unwrap();

    for collision in &player_controller_output.collisions {
        if enemy_entity_query.contains(collision.entity) {
            player_hit_event_writer.send(PlayerHitEvent(collision.entity));
        }
    }
}

fn handle_player_hit(
    mut player_hit_event_reader: EventReader<PlayerHitEvent>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    let mut _player_hit = false;
    for player_hit_event in &mut player_hit_event_reader {
        log::info!("Player hit by enemy: {:?}", player_hit_event.0);
        _player_hit = true;
        next_player_state.set(PlayerState::Dying);
        break;
    }
}

fn tick_death_timer(
    mut death_timer: ResMut<DeathTimer>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if death_timer.0.tick(time.delta()).just_finished() {
        for enemy in &enemy_query {
            commands.entity(enemy).despawn();
        }
        death_timer.0.reset();
        next_game_state.set(GameState::ManagingInventory);
    }
}
