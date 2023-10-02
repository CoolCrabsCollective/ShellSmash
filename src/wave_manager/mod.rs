use std::time::Duration;

use bevy::{log, prelude::*};
use bevy_rapier3d::prelude::*;
use queues::{IsQueue, Queue};
use rand::random;
use rand::Rng;

use crate::asset_loader::GameAssets;
use crate::config::SPAWN_ENEMIES;
use crate::enemy::{Enemy, EnemyBundle, EnemyType};
use crate::game_state::GameState;
use crate::item_spawner::spawn_random_item;
use crate::player::PlayerControllerState;
use crate::wave_manager::waves::{wave_generation, DEFINED_WAVES};

mod waves;

pub const ARENA_DIMENSIONS_METERS: [f32; 2] = [24.0, 30.0];

pub struct WaveManagerPlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum WaveState {
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

    jellyfish_count: i32,
    urchin_count: i32,
    shrimp_count: i32,

    luck: Option<i32>,

    drop_item_count: i32,
}

#[derive(Resource)]
pub struct Wave {
    pub count: i32,
    pub luck: Queue<i32>, // better items should drop as luck increases

    pub wave_definition: WaveDefinition,
}

impl Wave {
    fn new() -> Self {
        Self {
            count: 0,
            luck: Default::default(),
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

        app.add_systems(
            OnEnter(GameState::FightingInArena),
            show_ui.run_if(in_state(WaveState::ACTIVE_WAVE)),
        );
        app.add_systems(
            OnEnter(GameState::FightingInArena),
            show_ui.run_if(in_state(WaveState::ACTIVE_WAVE_SPAWNING)),
        );
        app.add_systems(OnExit(GameState::FightingInArena), hide_ui);

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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if start_delay_timer.0.tick(time.delta()).finished() {
        log::info!("Starting wave: {}", current_wave.count);

        next_state.set(WaveState::ACTIVE_WAVE_SPAWNING);
        start_delay_timer.0.reset();

        commands
            .spawn(NodeBundle {
                style: Style {
                    top: Val::Percent(0.0),
                    left: Val::Percent(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .insert(WaveUI)
            .with_children(|parent| {
                parent.spawn(
                    (TextBundle::from_section(
                        format!("Wave {:?}", current_wave.count + 1),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(5.0),
                        right: Val::Px(15.0),
                        ..default()
                    })),
                );
            });
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

    if current_wave.wave_definition.jellyfish_count <= 0
        && current_wave.wave_definition.urchin_count <= 0
        && current_wave.wave_definition.shrimp_count <= 0
    {
        next_state.set(WaveState::ACTIVE_WAVE);
        return;
    }

    if spawn_timer.0.tick(time.delta()).just_finished() {
        let player_transform = player_transform_query.single();

        let mut enemy_type = EnemyType::Jellyfish;
        let mut enemy_count = &mut current_wave.wave_definition.jellyfish_count;

        let mut attempts = 0;
        let mut selected = false;
        while !selected && attempts < 10 {
            attempts += 1;

            let mut rng = rand::thread_rng();
            let spawnTypeId = rng.gen_range(0..3);
            match spawnTypeId {
                0 => {
                    enemy_type = EnemyType::Jellyfish;
                    enemy_count = &mut current_wave.wave_definition.jellyfish_count;
                }
                1 => {
                    enemy_type = EnemyType::Urchin;
                    enemy_count = &mut current_wave.wave_definition.urchin_count;
                }
                2 => {
                    enemy_type = EnemyType::Shrimp;
                    enemy_count = &mut current_wave.wave_definition.shrimp_count;
                }
                _ => {}
            }

            if *enemy_count > 0 {
                selected = true;
            }
        }

        attempts = 0;

        // Spawn enemy
        let rand_x = random::<f32>() - 0.5;
        let rand_y = random::<f32>() - 0.5;

        let position = Vec3::new(
            ((rand_x) * ARENA_DIMENSIONS_METERS[0]) * 2.0,
            1.0,
            ((rand_y) * ARENA_DIMENSIONS_METERS[0]) * 2.0,
        );
        if (player_transform.translation - position).length() > 3.0 {
            commands.spawn(EnemyBundle::new(position, &game_assets, enemy_type));

            *enemy_count -= 1;
        }

        spawn_timer.0.reset();
    }
}

fn hide_ui(mut commands: Commands, wave_ui_query: Query<Entity, With<WaveUI>>) {
    // println!("Hiding Wave UI");
    for e in wave_ui_query.iter() {
        commands.entity(e).despawn();
    }
}

fn show_ui(mut commands: Commands, asset_server: Res<AssetServer>, current_wave: Res<Wave>) {
    // println!("Showing Wave UI");
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(WaveUI)
        .with_children(|parent| {
            parent.spawn(
                (TextBundle::from_section(
                    format!("Wave {:?}", current_wave.count + 1),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 48.0,
                        color: Color::BLACK,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                })),
            );
        });
}

fn check_for_wave_end(
    enemy_entity_query: Query<Entity, With<Enemy>>,
    mut current_wave: ResMut<Wave>,
    mut next_state: ResMut<NextState<WaveState>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    wave_ui_query: Query<Entity, With<WaveUI>>,
) {
    if enemy_entity_query.iter().len() <= 0 {
        log::info!("Ending wave: {}", current_wave.count);

        next_state.set(WaveState::WAVE_END);
        current_wave.count += 1;

        for e in wave_ui_query.iter() {
            commands.entity(e).despawn();
        }
        drop_items(&mut commands, meshes, materials, current_wave);
    }
}

fn drop_items(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut current_wave: ResMut<Wave>,
) {
    spawn_random_item(
        &mut current_wave.luck,
        commands,
        &mut meshes,
        &mut materials,
    );
}
#[derive(Component)]
struct WaveUI;

fn prepare_next_wave(
    mut start_delay_timer: ResMut<WaveStartDelayTimer>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut current_wave: ResMut<Wave>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    if current_wave.count < (DEFINED_WAVES.len() as i32) {
        // Wave count is within defined waves
        dbg!("using defined wave");
        current_wave.wave_definition = DEFINED_WAVES[current_wave.count as usize].clone();
    } else {
        current_wave.wave_definition = wave_generation(current_wave.count);
    }

    // set delay before next wave
    start_delay_timer.0.set_duration(Duration::from_secs_f32(
        current_wave.wave_definition.start_delay,
    ));

    spawn_timer.0.set_duration(Duration::from_secs_f32(
        current_wave.wave_definition.spawn_rate,
    ));

    let luck = current_wave.wave_definition.luck;
    if luck.is_some() {
        let _ = current_wave.luck.add(luck.unwrap());
    }

    next_state.set(WaveState::WAVE_START);
}
