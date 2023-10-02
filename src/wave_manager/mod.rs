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

pub const ARENA_DIMENSIONS_METERS: [f32; 2] = [24.0, 30.0];

pub struct WaveManagerPlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum WaveState {
    #[default]
    NO_WAVE,
    ACTIVE_WAVE,
    WAVE_END,
}

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct WaveStartDelayTimer(Timer);

impl Plugin for WaveManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<WaveState>();
        app.add_systems(OnEnter(GameState::FightingInArena), reset_start_delay);

        app.add_systems(
            Update,
            wait_for_wave_start
                .run_if(in_state(GameState::FightingInArena))
                .run_if(in_state(WaveState::NO_WAVE)),
        );
        app.add_systems(
            Update,
            spawn_enemies
                .run_if(in_state(GameState::FightingInArena))
                .run_if(in_state(WaveState::ACTIVE_WAVE)),
        );

        app.insert_resource(WaveStartDelayTimer(Timer::from_seconds(
            2.0,
            TimerMode::Once,
        )));
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
    }
}

fn reset_start_delay(mut start_delay_timer: ResMut<WaveStartDelayTimer>) {
    // start_delay_timer.0.reset();
}

fn wait_for_wave_start(
    mut start_delay_timer: ResMut<WaveStartDelayTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    if start_delay_timer.0.tick(time.delta()).finished() {
        next_state.set(WaveState::ACTIVE_WAVE)
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut start_delay_timer: ResMut<WaveStartDelayTimer>,
    mut spawn_timer: ResMut<SpawnTimer>,
    player_transform_query: Query<&Transform, With<PlayerControllerState>>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    if !SPAWN_ENEMIES {
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
            }
        }
    }
}
