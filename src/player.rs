use bevy::input::keyboard::KeyboardInput;
use bevy::math::vec3;
use bevy::window::PrimaryWindow;
use bevy::{log, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::enemy::Enemy;
use crate::game_state::GameState;
use crate::inventory::InventoryItem;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerControllerState {
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    is_I_pressed: bool,
    was_I_pressed: bool,

    is_K_pressed: bool,
    was_K_pressed: bool,

    velocity: Vec3,
}

type WomanHitByPlayer = Entity;

#[derive(Event)]
pub struct PlayerHitEvent(WomanHitByPlayer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            process_inputs.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_systems(
            Update,
            player_movement.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_systems(
            Update,
            detect_player_hit.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_systems(
            Update,
            handle_player_hit.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_event::<PlayerHitEvent>();
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
        .insert(PlayerControllerState::new())
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 1.0, 0.0)));
}

impl PlayerControllerState {
    fn new() -> Self {
        Self {
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,

            is_I_pressed: false,
            was_I_pressed: false,

            is_K_pressed: false,
            was_K_pressed: false,

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
            Some(KeyCode::I) => {
                state.is_I_pressed = event.state.is_pressed();
            }
            Some(KeyCode::K) => {
                state.is_K_pressed = event.state.is_pressed();
            }
            _ => {}
        }
    }
}

fn player_movement(
    mut commands: Commands,
    mut controllers: Query<&mut KinematicCharacterController, With<PlayerControllerState>>,
    time: Res<Time>,
    mut state: Query<&mut PlayerControllerState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut transform: Query<&mut Transform, With<PlayerControllerState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    if state.is_I_pressed && !state.was_I_pressed {
        let boomerang = InventoryItem::from((
            (1, 3, 3),
            vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
            Color::rgba(1.0, 1.0, 1.0, 1.0),
        ));

        boomerang.create_world_entity(transform.translation, false, commands, meshes, materials);
    } else if state.is_K_pressed && !state.was_K_pressed {
        let boomerang = InventoryItem::from((
            (1, 3, 3),
            vec![(0, 0, 0), (0, 0, 1), (0, 0, 2), (-1, 0, 0), (-2, 0, 0)],
            Color::rgba(1.0, 1.0, 1.0, 1.0),
        ));

        boomerang.create_world_entity(transform.translation, true, commands, meshes, materials);
    }

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
    state.was_I_pressed = state.is_I_pressed;
    state.was_K_pressed = state.is_K_pressed;
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

fn handle_player_hit(mut player_hit_event_reader: EventReader<PlayerHitEvent>) {
    for player_hit_event in &mut player_hit_event_reader {
        log::info!("Player hit by enemy: {:?}", player_hit_event.0);
    }
}

fn check_for_items(
    items: Query<&InventoryItem>,
    mut controllers: Query<&mut KinematicCharacterController, With<PlayerControllerState>>,
) {
    let mut near_items: Vec<&InventoryItem> = vec![];
    let current_location = controllers.single_mut().translation;

    if current_location.is_some() {
        let unwrap_location = current_location.unwrap();

        for item in items.iter() {
            if item.intersects(IVec3 {
                x: unwrap_location.x as i32,
                y: unwrap_location.y as i32,
                z: unwrap_location.z as i32,
            }) {
                near_items.push(item);
            }
        }
    }

    if near_items.len() > 0 {
        log::info!("Items found: {:?}", near_items)
    }
}
