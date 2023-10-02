use std::time::Duration;

use bevy::prelude::Projection::Perspective;
use bevy::prelude::*;
use bevy::render::camera;

use super::gizmo::highlight_gizmo;
use crate::config::{DEFAULT_BAG_LOCATION, INVENTORY_GRID_DIMENSIONS};
use crate::game::HolyCam;
use crate::game_state::GameState;
use crate::inventory::gizmo::update_gizmo_position;
use crate::inventory::selection::SelectedItem;
use crate::inventory::{InventoryData, InventoryItem, PackedInventoryItem};

pub struct InventoryControllerPlugin;

impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            update_gizmo_position
                .after(update_cube_rotation)
                .run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            update_cube_rotation.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            move_inventory_items.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            update_inventory_data.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(Update, highlight_gizmo);
        app.add_systems(OnEnter(GameState::ManagingInventory), set_fov);
        app.insert_resource(InventoryControllerState::new());
        app.insert_resource(CubeRotationAnime::new());
    }
}

#[derive(Component)]
pub struct VoxelCoordinateFrame;

fn setup(mut commands: Commands) {
    commands.spawn((
        VoxelCoordinateFrame,
        SpatialBundle::from(Transform {
            translation: DEFAULT_BAG_LOCATION,
            ..default()
        }),
    ));
}

#[derive(Resource, Debug)]
pub struct CubeRotationAnime {
    pub enabled: bool,
    pub anime_time: Timer,
    pub start_rotation: f32,
    pub end_rotation: f32,
}

impl CubeRotationAnime {
    fn new() -> CubeRotationAnime {
        CubeRotationAnime {
            enabled: false,
            anime_time: Timer::new(Duration::from_millis(750), TimerMode::Once),
            start_rotation: 0.0,
            end_rotation: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct InventoryControllerState {
    pub view_index: usize,
}

impl InventoryControllerState {
    pub fn new() -> Self {
        Self { view_index: 0 }
    }
}

fn update_cube_rotation(
    key_codes: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut rotation_anime: ResMut<CubeRotationAnime>,
    mut state: ResMut<InventoryControllerState>,
    mut param_set: ParamSet<(
        Query<&Transform, With<VoxelCoordinateFrame>>,
        Query<&mut Transform, With<HolyCam>>,
    )>,
) {
    let camera_trans = {
        let camera_query = param_set.p1();
        camera_query.single().translation
    };
    let vox_trans = {
        let vox_query = param_set.p0();
        vox_query.single().translation
    };
    let deg = rotation_anime.end_rotation;
    let mut camera_xz: Vec2 = 8.0 * Vec2::from_angle((deg as f32).to_radians());
    let mut camera_y = camera_trans.y;
    if rotation_anime.enabled {
        rotation_anime.anime_time.tick(time.delta());
        rotation_anime.enabled = !rotation_anime.anime_time.finished();
        let progress = rotation_anime.anime_time.percent();
        let parameterized_progress = 1.0 / (1.0 + f32::exp(-12.0 * (progress - 0.5)));
        let rotation_angle = rotation_anime.end_rotation - rotation_anime.start_rotation;

        let angle = rotation_anime.start_rotation + parameterized_progress * rotation_angle;
        camera_xz = 8.0 * Vec2::from_angle((angle as f32).to_radians());
    } else {
        let mut start_anime: bool = false;
        let mut rotation_change = 0.0;
        if key_codes.just_pressed(KeyCode::Left) {
            state.view_index = if state.view_index == 0 {
                3
            } else {
                state.view_index - 1
            };
            start_anime = true;
            rotation_change = -90.0;
        } else if key_codes.just_pressed(KeyCode::Right) {
            state.view_index = (state.view_index + 1) % 4;
            start_anime = true;
            rotation_change = 90.0;
        }

        if start_anime {
            rotation_anime.enabled = true;
            rotation_anime.start_rotation = rotation_anime.end_rotation;
            rotation_anime.end_rotation = rotation_anime.start_rotation + rotation_change;
            rotation_anime.anime_time.reset();
        }

        let increment = 2.0;
        let max_increment = 10.0;
        let mut change = 0.0;
        if key_codes.just_pressed(KeyCode::Up) {
            change = change + increment;
        } else if key_codes.just_pressed(KeyCode::Down) {
            change = change - increment;
        }

        camera_y = camera_trans.y + change;
        camera_y = camera_y.max(-max_increment).min(max_increment);
    }
    let mut camera_query = param_set.p1();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = vox_trans + Vec3::from((camera_xz.x, camera_y, camera_xz.y));
    let look_at_my_balls = camera_transform.looking_at(vox_trans, Vec3::Y);
    camera_transform.rotation = look_at_my_balls.rotation;
}

pub fn update_inventory_data(query: Query<&PackedInventoryItem>, mut inv: ResMut<InventoryData>) {
    let mut items: Vec<InventoryItem> = Vec::new();
    for p in query.iter() {
        items.push(p.data.clone())
    }
    inv.grid = InventoryData::grid_from_items(items, IVec3::from_array(INVENTORY_GRID_DIMENSIONS))
}

#[derive(Debug, Copy, Clone)]
pub enum ItemDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    FORWARD,
    BACKWARDS,
}

pub fn move_item(item: &mut PackedInventoryItem, item_dir: ItemDirection, view_index: usize) {
    let trans: Vec<IVec3> = vec![
        IVec3::from((0, 0, -1)),
        IVec3::from((-1, 0, 0)),
        IVec3::from((0, 0, 1)),
        IVec3::from((1, 0, 0)),
    ];

    match item_dir {
        ItemDirection::LEFT => {
            item.data.translate(
                trans[if 2 <= view_index {
                    (6 - view_index) % 4
                } else {
                    2 - view_index
                }],
            );
        }
        ItemDirection::RIGHT => item.data.translate(trans[(4 - view_index) % 4]),
        ItemDirection::UP => {
            item.data.translate(IVec3::from((0, 1, 0)));
        }
        ItemDirection::DOWN => {
            item.data.translate(IVec3::from((0, -1, 0)));
        }
        ItemDirection::BACKWARDS => {
            item.data.translate(
                trans[if 3 <= view_index {
                    (7 - view_index) % 4
                } else {
                    3 - view_index
                }],
            );
        }
        ItemDirection::FORWARD => {
            item.data.translate(
                trans[if 1 <= view_index {
                    (5 - view_index) % 4
                } else {
                    1 - view_index
                }],
            );
        }
        _ => {}
    }
}

fn move_inventory_items(
    state: Res<InventoryControllerState>,
    key_codes: Res<Input<KeyCode>>,
    mut query_items: Query<(Entity, &mut PackedInventoryItem)>,
    selected: Res<SelectedItem>,
) {
    if key_codes.just_pressed(KeyCode::S) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                move_item(&mut item.1, ItemDirection::BACKWARDS, state.view_index);
            }
        }
    } else if key_codes.just_pressed(KeyCode::D) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                move_item(&mut item.1, ItemDirection::RIGHT, state.view_index);
            }
        }
    } else if key_codes.just_pressed(KeyCode::W) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                move_item(&mut item.1, ItemDirection::FORWARD, state.view_index);
            }
        }
    } else if key_codes.just_pressed(KeyCode::A) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                move_item(&mut item.1, ItemDirection::LEFT, state.view_index);
            }
        }
    } else if key_codes.just_pressed(KeyCode::Q) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                item.1.data.rotate(true);
            }
        }
    } else if key_codes.just_pressed(KeyCode::E) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                item.1.data.rotate(false);
            }
        }
    } else if key_codes.just_pressed(KeyCode::Z) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                move_item(&mut item.1, ItemDirection::UP, state.view_index);
            }
        }
    } else if key_codes.just_pressed(KeyCode::X) {
        for mut item in query_items.iter_mut() {
            if Some(item.0) == selected.selected_entity {
                move_item(&mut item.1, ItemDirection::DOWN, state.view_index);
            }
        }
    }
}

fn set_fov(mut camera_query: Query<(&mut Transform, &mut Projection)>) {
    let mut proj = camera_query.single_mut().1;

    if let Perspective(pers_proj) = proj.as_mut() {
        pers_proj.fov = 45.0f32.to_radians();
    }
}
