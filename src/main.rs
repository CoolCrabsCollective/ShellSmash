use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;

use crate::game::GamePlugin;
use crate::inventory::InventoryPlugin;

mod debug_camera_controller;
mod game;
mod inventory;
mod inventory_controller;
mod item_mesh_generator;
mod level_loader;
mod math;
mod voxel_renderer;
mod wall;

const USE_DEBUG_CAM: bool = false;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Game,
    Inventory,
}

impl GameState {
    fn turn(&self) -> Self {
        match self {
            GameState::Game => GameState::Inventory,
            GameState::Inventory => GameState::Game,
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(RenderPlugin {
        wgpu_settings: WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        },
    }));
    app.add_plugins(GamePlugin);
    app.add_plugins(InventoryPlugin);
    app.add_state::<GameState>()
        .add_systems(Update, (bevy::window::close_on_esc, swap_controls));

    app.run();
}

fn swap_controls(
    k_input: Res<Input<KeyCode>>,
    current_game_state: ResMut<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if k_input.just_pressed(KeyCode::Space) {
        let new_status = current_game_state.get().turn();

        next_game_state.set(new_status);
    }
}
