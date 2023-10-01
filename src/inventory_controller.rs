use crate::config::INVENTORY_GRID_DIMENSIONS;
use crate::game_state::GameState;
use crate::inventory::{InventoryData, InventoryItem};
use crate::math::deg_to_rad;
use crate::voxel_renderer::VoxelCoordinateFrame;
use bevy::{log, prelude::*};
use std::time::Duration;

pub struct InventoryControllerPlugin;

impl Plugin for InventoryControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ManagingInventory), enter_inventory);
        app.add_systems(
            Update,
            update_camera_position.run_if(in_state(GameState::ManagingInventory)),
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
        app.insert_resource(InventoryControllerState::new());
        app.insert_resource(CubeRotationAnime::new());
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ControlledOrientation {
    horizontal: f32,
    vertical: f32,
    zoom_pos: f32,
}

impl ControlledOrientation {
    pub fn to_quat(self) -> Quat {
        Quat::from_euler(EulerRot::XYZ, 0.0, self.horizontal, 0.0)
            * Quat::from_euler(EulerRot::XYZ, self.vertical, 0.0, 0.0)
    }
}

#[derive(Resource, Debug)]
struct CubeRotationAnime {
    enabled: bool,
    anime_time: Timer,
    start_rotation: f32,
    end_rotation: f32,
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
struct InventoryControllerState {
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
    mut query: Query<&mut Transform, With<VoxelCoordinateFrame>>,
) {
    if rotation_anime.enabled {
        rotation_anime.anime_time.tick(time.delta());
        rotation_anime.enabled = !rotation_anime.anime_time.finished();
        let progress = rotation_anime.anime_time.percent();
        let parameterized_progress = 1.0 / (1.0 + f32::exp(-12.0 * (progress - 0.5)));
        let rotation_angle = rotation_anime.end_rotation - rotation_anime.start_rotation;

        let mut vox = query.single_mut();
        let quat = Quat::from_rotation_y(deg_to_rad(
            rotation_angle + parameterized_progress * rotation_angle,
        ));
        vox.rotation = quat;
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
            let vox = query.single();
            rotation_anime.enabled = true;
            let current_rot = vox.rotation.to_euler(EulerRot::XYZ).1;
            rotation_anime.start_rotation = current_rot;
            rotation_anime.end_rotation = current_rot + rotation_change;
            rotation_anime.anime_time.reset();
        }
        // vox.rotation = Quat::from_rotation_y(deg_to_rad(possible_rotations[state.view_index]));
    }
}

fn update_camera_position(
    mut param_set: ParamSet<(
        Query<&Transform, With<VoxelCoordinateFrame>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let cam_distance = 8.0;
    let vox_trans = {
        let vox_trans_query = param_set.p0();
        vox_trans_query.single().translation
    };
    let mut camera_translation_query = param_set.p1();
    let mut camera_translation = camera_translation_query.single_mut();
    camera_translation.translation = vox_trans + Vec3::from((0.0, cam_distance, cam_distance));
    let look_at_my_balls = camera_translation.looking_at(vox_trans, Vec3::Y);
    camera_translation.rotation = look_at_my_balls.rotation;
}

pub fn update_inventory_data(query: Query<&InventoryItem>, mut inv: ResMut<InventoryData>) {
    let mut items: Vec<InventoryItem> = Vec::new();
    for p in query.iter() {
        items.push(p.clone())
    }
    inv.grid = InventoryData::grid_from_items(items, IVec3::from_array(INVENTORY_GRID_DIMENSIONS));
}

fn get_initial_camera_transform() -> Transform {
    Transform::default()
        .with_translation(Vec3::new(500.0, 8.0, 8.0))
        .looking_at(Vec3::new(500.0, 0.0, 0.0), Vec3::Y)
}

fn enter_inventory(mut cam_transform_query: Query<&mut Transform, With<Camera>>) {
    (*cam_transform_query.single_mut()) = get_initial_camera_transform();
}

fn move_inventory_items(
    state: Res<InventoryControllerState>,
    key_codes: Res<Input<KeyCode>>,
    mut query_items: Query<&mut InventoryItem>,
) {
    let trans: Vec<IVec3> = vec![
        IVec3::from((0, 0, -1)),
        IVec3::from((-1, 0, 0)),
        IVec3::from((0, 0, 1)),
        IVec3::from((1, 0, 0)),
    ];
    let view_index = state.view_index;
    if key_codes.just_pressed(KeyCode::W) {
        for mut item in query_items.iter_mut() {
            item.translate(trans[(4 - view_index) % 4]);
        }
    } else if key_codes.just_pressed(KeyCode::A) {
        for mut item in query_items.iter_mut() {
            item.translate(
                trans[if 1 <= view_index {
                    (5 - view_index) % 4
                } else {
                    1 - view_index
                }],
            );
        }
    } else if key_codes.just_pressed(KeyCode::S) {
        for mut item in query_items.iter_mut() {
            item.translate(
                trans[if 2 <= view_index {
                    (6 - view_index) % 4
                } else {
                    2 - view_index
                }],
            );
        }
    } else if key_codes.just_pressed(KeyCode::D) {
        for mut item in query_items.iter_mut() {
            item.translate(
                trans[if 3 <= view_index {
                    (7 - view_index) % 4
                } else {
                    3 - view_index
                }],
            );
        }
    }
}
