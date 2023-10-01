use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use game_state::GameStatePlugin;

use crate::game::GamePlugin;
use crate::inventory::InventoryPlugin;

mod combat;
mod debug_camera_controller;
mod game;
mod game_state;
mod health;
mod inventory;
mod inventory_controller;
mod item_mesh_generator;
mod item_spawner;
mod level_loader;
mod math;
mod player;
mod voxel_renderer;
mod wall;
mod world_item;

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
    app.add_plugins(GameStatePlugin);
    app.add_systems(Update, bevy::window::close_on_esc);

    app.run();
}
