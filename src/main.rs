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
mod player;
mod projectile;
mod title_screen;
mod wave_manager;
mod world_item;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, FilterMode, SamplerDescriptor};
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use game_state::GameStatePlugin;
use title_screen::TitleScreenPlugin;

use crate::game::GamePlugin;
use crate::inventory::InventoryPlugin;

fn main() {
    let mut app = App::new();

    let default_sampler = SamplerDescriptor {
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        address_mode_w: AddressMode::Repeat,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        mipmap_filter: FilterMode::Linear,
        ..default()
    };
    if cfg!(target_arch = "wasm32") {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin { default_sampler }),
        );
    } else {
        app.add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    },
                })
                .set(ImagePlugin { default_sampler }),
        );
    }

    app.add_plugins(TitleScreenPlugin);
    app.add_plugins(GamePlugin);
    app.add_plugins(InventoryPlugin);
    app.add_plugins(GameStatePlugin);
    app.add_systems(Update, bevy::window::close_on_esc);

    app.run();
}
