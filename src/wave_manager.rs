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

pub const ARENA_DIMENSIONS_METERS: [f32; 2] = [24.0, 30.0];

pub struct WaveManagerPlugin;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct WaveStartDelayTimer(Timer);

impl Plugin for WaveManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::FightingInArena), reset_start_delay);
        app.add_systems(
            Update,
            spawn_enemies.run_if(in_state(GameState::FightingInArena)),
        );

        app.insert_resource(WaveStartDelayTimer(Timer::from_seconds(
            2.0,
            TimerMode::Once,
        )));
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
    }
}

fn reset_start_delay(mut start_delay_timer: ResMut<WaveStartDelayTimer>) {
    start_delay_timer.0.reset();
}

fn spawn_enemies(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut start_delay_timer: ResMut<WaveStartDelayTimer>,
    mut spawn_timer: ResMut<SpawnTimer>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    if !SPAWN_ENEMIES {
        return;
    }

    if !start_delay_timer.0.tick(time.delta()).finished() {
        return;
    }

    if spawn_timer.0.tick(time.delta()).just_finished() {
        let position = Vec3::new(
            (random::<f32>() - 0.5) * ARENA_DIMENSIONS_METERS[0],
            1.0,
            (random::<f32>() - 0.5) * ARENA_DIMENSIONS_METERS[0],
        );
        commands.spawn(EnemyBundle::new(position, game_assets));
    }
}
