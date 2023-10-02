pub(crate) mod combat;

use bevy::audio::PlaybackMode::Despawn;
use bevy::audio::Volume::Relative;
use bevy::audio::VolumeLevel;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::math::vec3;
use bevy::window::PrimaryWindow;
use bevy::{log, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::config::{
    COLLISION_GROUP_ENEMIES, COLLISION_GROUP_PLAYER, COLLISION_GROUP_PROJECTILES,
    COLLISION_GROUP_TERRAIN, COLLISION_GROUP_WALLS,
};
use crate::enemy::{Enemy, ENEMY_COLLIDER_RADIUS};
use crate::game::HolyCam;
use crate::game_camera_controller::GameCameraControllerPlugin;
use crate::game_state::GameState;
use crate::inventory::{Inventory, ItemType};
use crate::player::combat::PlayerCombatState;
use crate::player::combat::{PlayerCombatPlugin, PLAYER_INVICIBILITY_COOLDOWN};
use crate::projectile::{Projectile, ProjectileBundle};
use crate::wave_manager::{Wave, WaveState};
use crate::world_item::WeaponHolder;

use self::combat::BASE_ATTACK_COOLDOWN;

pub const PLAYER_HEIGHT: f32 = 0.6;
pub const PLAYER_WIDTH: f32 = 0.5;
pub const PLAYER_SHOOTING_PROJECTILE_CUBE_HALF_SIZE: f32 = 0.1;

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

    is_shoot_pressed: bool,

    pub velocity: Vec3,
}

#[derive(Event)]
pub struct PlayerHitEvent(Entity);

#[derive(Resource)]
struct DeathTimer(Timer);

#[derive(Resource)]
struct PlayerShootingState {
    rate_limiter: Option<Timer>,
    mesh_material_handle: Option<(Handle<Mesh>, Handle<StandardMaterial>)>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(GameState::FightingInArena), set_player_active);
        app.add_systems(Update, process_inputs);
        app.add_systems(
            Update,
            player_movement.run_if(
                in_state(GameState::FightingInArena).and_then(in_state(PlayerState::Fighting)),
            ),
        );
        app.add_systems(
            Update,
            player_shooting.run_if(
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
        app.add_plugins((GameCameraControllerPlugin, PlayerCombatPlugin));
        app.insert_resource(PlayerShootingState {
            rate_limiter: None,
            mesh_material_handle: None,
        });
    }
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut shooting_state: ResMut<PlayerShootingState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Collider::capsule_y(0.3, 0.5))
        .insert(SceneBundle {
            scene: asset_server.load("hermit.glb#Scene0"),
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
            filter_groups: Some(CollisionGroups {
                // memberships: COLLISION_GROUP_PLAYER,
                // filters: COLLISION_GROUP_TERRAIN,
                memberships: COLLISION_GROUP_PLAYER,
                filters: COLLISION_GROUP_TERRAIN
                    | COLLISION_GROUP_PROJECTILES
                    | COLLISION_GROUP_WALLS,
            }),
            ..default()
        })
        .insert(PlayerControllerState::new())
        .insert(PlayerCombatState::new())
        .insert(WeaponHolder {
            current_weapon: None,
        })
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 1.0, 0.0)));

    let bubble: Mesh = Mesh::try_from(shape::Icosphere {
        radius: PLAYER_SHOOTING_PROJECTILE_CUBE_HALF_SIZE * 2.0,
        subdivisions: 3,
    })
    .unwrap();
    shooting_state.mesh_material_handle = Some((
        meshes.add(bubble),
        materials.add(Color::rgb(0.05, 0.4, 0.9).into()),
    ));
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

            is_shoot_pressed: false,

            velocity: vec3(0.0, 0.0, 0.0),
        }
    }
}

fn process_inputs(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut mouse_input_events: EventReader<MouseButtonInput>,
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

    for event in mouse_input_events.iter() {
        match event.button {
            MouseButton::Left => {
                state.is_shoot_pressed = event.state == ButtonState::Pressed;
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
    camera_q: Query<(&Camera, &GlobalTransform), With<HolyCam>>,
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

fn player_shooting(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut shooting_state: ResMut<PlayerShootingState>,
    player_transform_query: Query<(
        &Transform,
        &PlayerControllerState,
        &PlayerCombatState,
        &WeaponHolder,
    )>,
    time: Res<Time>,
) {
    let (player_transform, player_controller_state, player_combat_state, weapon_holder_state) =
        player_transform_query.single();

    if !player_controller_state.is_shoot_pressed {
        return;
    }

    if weapon_holder_state.current_weapon.is_none() {
        return;
    }

    let (_, current_weapon) = weapon_holder_state.current_weapon.as_ref().unwrap();

    if current_weapon.item_type != ItemType::RANGED_WEAPON {
        return;
    }

    let point_in_front_of_player =
        player_transform.translation + player_transform.forward().normalize() * 1.0;

    //  Timer::from_seconds(PLAYER_SHOOTING_RATE_PERIOD, TimerMode::Repeating)
    if player_controller_state.is_shoot_pressed && weapon_holder_state.current_weapon.is_some() {
        if shooting_state.rate_limiter.is_none()
            || shooting_state
                .rate_limiter
                .as_mut()
                .unwrap()
                .tick(time.delta())
                .just_finished()
        {
            commands.spawn(AudioBundle {
                source: asset_server.load("shoot.ogg"),
                settings: PlaybackSettings {
                    mode: Despawn,
                    volume: Relative(VolumeLevel::new(1.0f32)),
                    ..default()
                },
                ..default()
            });
            commands.spawn(ProjectileBundle {
                pbr: PbrBundle {
                    mesh: shooting_state
                        .mesh_material_handle
                        .as_ref()
                        .unwrap()
                        .0
                        .clone(),
                    material: shooting_state
                        .mesh_material_handle
                        .as_ref()
                        .unwrap()
                        .1
                        .clone(),
                    transform: Transform::from_translation(Vec3::new(
                        point_in_front_of_player.x,
                        ENEMY_COLLIDER_RADIUS,
                        point_in_front_of_player.z,
                    )),
                    ..default()
                },
                projectile: Projectile {
                    speed: current_weapon.projectile_speed,
                    direction: player_transform.forward(),
                    source_weapon: current_weapon.clone(),
                },
                collider: Collider::cuboid(
                    PLAYER_SHOOTING_PROJECTILE_CUBE_HALF_SIZE * 2.0,
                    PLAYER_SHOOTING_PROJECTILE_CUBE_HALF_SIZE * 2.0,
                    PLAYER_SHOOTING_PROJECTILE_CUBE_HALF_SIZE * 2.0,
                ),
                collision_groups: CollisionGroups {
                    memberships: COLLISION_GROUP_PROJECTILES,
                    filters: COLLISION_GROUP_ENEMIES,
                },
            });
        }

        if shooting_state.rate_limiter.is_none() {
            shooting_state.rate_limiter = Some(Timer::from_seconds(
                BASE_ATTACK_COOLDOWN
                    / (player_combat_state.attack_speed * current_weapon.weapon_attack_speed),
                TimerMode::Repeating,
            ))
        }
    } else {
        shooting_state.rate_limiter = None;
    }
}

fn vertical_vel_reset(
    mut player_controller_output_query: Query<(
        &KinematicCharacterControllerOutput,
        &mut PlayerControllerState,
    )>,
) {
    let (player_controller_output, mut player_state) =
        player_controller_output_query.get_single_mut().unwrap();

    if player_controller_output.grounded {
        player_state.velocity.y = 0.0;
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
    mut player_state: Query<&mut PlayerCombatState>,
    mut player_hit_event_reader: EventReader<PlayerHitEvent>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    time: Res<Time>,
) {
    let mut state = player_state.single_mut();

    if state.last_hit + PLAYER_INVICIBILITY_COOLDOWN > time.elapsed_seconds() {
        return;
    }

    for player_hit_event in &mut player_hit_event_reader {
        log::info!("Player hit by enemy: {:?}", player_hit_event.0);

        state.current_hp -= 1;
        state.last_hit = time.elapsed_seconds();
        state.last_heal = time.elapsed_seconds();

        if state.current_hp == 0 {
            next_player_state.set(PlayerState::Dying);
            break;
        } else {
            break;
        }
    }
}

fn tick_death_timer(
    mut death_timer: ResMut<DeathTimer>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    mut wave: ResMut<Wave>,
    mut next_wave_state: ResMut<NextState<WaveState>>,
    mut inventory: ResMut<Inventory>,
) {
    if death_timer.0.tick(time.delta()).just_finished() {
        for enemy in &enemy_query {
            commands.entity(enemy).despawn();
        }
        death_timer.0.reset();
        next_game_state.set(GameState::TitleScreen);
        wave.count = 0;
        wave.luck = 0;
        next_wave_state.set(WaveState::WAVE_END);
        inventory.content = Vec::new();
    }
}
