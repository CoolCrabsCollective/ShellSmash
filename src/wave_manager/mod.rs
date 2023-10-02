mod waves;

use std::time::Duration;

use bevy::math::vec3;
use bevy::window::PrimaryWindow;
use bevy::{log, prelude::*};
use bevy_rapier3d::prelude::*;
use rand::random;

use crate::asset_loader::GameAssets;
use crate::config::SPAWN_ENEMIES;
use crate::enemy::{Enemy, EnemyBundle};
use crate::game_state::GameState;
use crate::player::PlayerControllerState;
use crate::wave_manager::waves::DEFINED_WAVES;

pub const ARENA_DIMENSIONS_METERS: [f32; 2] = [24.0, 30.0];

pub struct WaveManagerPlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
enum WaveState {
    #[default]
    WAVE_START,
    ACTIVE_WAVE_SPAWNING,
    ACTIVE_WAVE,
    WAVE_END,
}

#[derive(Clone)]
pub struct WaveDefinition {
    start_delay: f32,
    spawn_rate: f32,

    enemy_count: i32,
    shrimp_count: i32,

    drop_item_count: i32,
}

#[derive(Resource)]
struct Wave {
    count: i32,
    luck: i32, // better items should drop as luck increases

    wave_definition: WaveDefinition,
}

impl Wave {
    fn new() -> Self {
        Self {
            count: 0,
            luck: 0,
            wave_definition: DEFINED_WAVES[0].clone(),
        }
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct WaveStartDelayTimer(Timer);

impl Plugin for WaveManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<WaveState>();

        app.add_systems(
            Update,
            wait_for_wave_start
                .run_if(in_state(GameState::FightingInArena))
                .run_if(in_state(WaveState::WAVE_START)),
        );
        app.add_systems(
            Update,
            spawn_enemies
                .run_if(in_state(GameState::FightingInArena))
                .run_if(in_state(WaveState::ACTIVE_WAVE_SPAWNING)),
        );
        app.add_systems(
            Update,
            check_for_wave_end
                .run_if(in_state(GameState::FightingInArena))
                .run_if(in_state(WaveState::ACTIVE_WAVE)),
        );
        app.add_systems(
            Update,
            prepare_next_wave
                .run_if(in_state(GameState::FightingInArena))
                .run_if(in_state(WaveState::WAVE_END)),
        );

        app.insert_resource(Wave::new());

        app.insert_resource(WaveStartDelayTimer(Timer::from_seconds(
            2.0,
            TimerMode::Once,
        )));
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            DEFINED_WAVES[0].start_delay,
            TimerMode::Repeating,
        )));
    }
}

fn wait_for_wave_start(
    mut start_delay_timer: ResMut<WaveStartDelayTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<WaveState>>,
    current_wave: ResMut<Wave>,
) {
    if start_delay_timer.0.tick(time.delta()).finished() {
        log::info!("Starting wave: {}", current_wave.count);

        next_state.set(WaveState::ACTIVE_WAVE_SPAWNING);
        start_delay_timer.0.reset();
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    player_transform_query: Query<&Transform, With<PlayerControllerState>>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
    mut current_wave: ResMut<Wave>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    if !SPAWN_ENEMIES {
        return;
    }

    if current_wave.wave_definition.enemy_count <= 0 {
        next_state.set(WaveState::ACTIVE_WAVE);
        return;
    }

    if spawn_timer.0.tick(time.delta()).just_finished() {
        let mut attempts = 0;
        let mut spawned = false;

        let player_transform = player_transform_query.single();

        while !spawned && attempts < 10 {
            attempts += 1;

            let position = Vec3::new(
                (random::<f32>() - 0.5) * ARENA_DIMENSIONS_METERS[0],
                1.0,
                (random::<f32>() - 0.5) * ARENA_DIMENSIONS_METERS[0],
            );
            if (player_transform.translation - position).length() > 3.0 {
                commands.spawn(EnemyBundle::new(position, &game_assets));
                spawned = true;

                current_wave.wave_definition.enemy_count -= 1;
            }
        }

        spawn_timer.0.reset();
    }
}

fn check_for_wave_end(
    enemy_entity_query: Query<Entity, With<Enemy>>,
    mut current_wave: ResMut<Wave>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    if enemy_entity_query.iter().len() <= 0 {
        log::info!("Ending wave: {}", current_wave.count);

        next_state.set(WaveState::WAVE_END);
        current_wave.count += 1;
    }
}

fn prepare_next_wave(
    mut start_delay_timer: ResMut<WaveStartDelayTimer>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut current_wave: ResMut<Wave>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    if current_wave.count < (DEFINED_WAVES.len() as i32) {
        // Wave count is within defined waves
        current_wave.wave_definition = DEFINED_WAVES[current_wave.count as usize].clone();

        // set delay before next wave
        start_delay_timer.0.set_duration(Duration::from_secs_f32(
            current_wave.wave_definition.start_delay,
        ));

        spawn_timer.0.set_duration(Duration::from_secs_f32(
            current_wave.wave_definition.spawn_rate,
        ))
    }

    next_state.set(WaveState::WAVE_START);
}
