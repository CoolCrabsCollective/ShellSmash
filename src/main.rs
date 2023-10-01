mod asset_loader;
mod collectable;
mod config;
mod debug_camera_controller;
mod enemy;
mod game;
mod game_camera_controller;
mod game_state;
mod inventory;
mod item_mesh_generator;
mod item_spawner;
mod level_loader;
mod math;
mod player;
mod voxel_renderer;
mod wall;
mod wave_manager;
mod world_item;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use game_state::GameStatePlugin;

use crate::game::GamePlugin;
use crate::inventory::InventoryPlugin;

fn main() {
    let mut app = App::new();

    if cfg!(target_arch = "wasm32") {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }));
    } else {
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            },
        }));
    }

    app.add_plugins(AssetLoaderPlugin);
    app.add_plugins(GamePlugin);
    app.add_plugins(InventoryPlugin);
    app.add_plugins(GameStatePlugin);
    app.add_systems(Update, bevy::window::close_on_esc);

    app.run();
}
