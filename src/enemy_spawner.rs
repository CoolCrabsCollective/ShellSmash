use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use rand::random;

use crate::config::SPAWN_ENEMIES;
use crate::enemy::EnemyBundle;
use crate::game_state::GameState;

pub const ARENA_DIMENSIONS_METERS: [f32; 2] = [24.0, 30.0];

pub struct EnemySpawnerPlugin;

#[derive(Resource)]
struct SpawnTimer(Timer);

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_enemies.run_if(in_state(GameState::FightingInArena)),
        );
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut asset_server: ResMut<AssetServer>,
) {
    if !SPAWN_ENEMIES {
        return;
    }

    if spawn_timer.0.tick(time.delta()).just_finished() {
        let position = Vec3::new(
            (random::<f32>() - 0.5) * ARENA_DIMENSIONS_METERS[0],
            1.0,
            (random::<f32>() - 0.5) * ARENA_DIMENSIONS_METERS[0],
        );
        commands.spawn(EnemyBundle::new(&mut asset_server, position));
    }
}
