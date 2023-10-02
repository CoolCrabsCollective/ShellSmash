use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_mod_raycast::ray_intersection_over_mesh;
use bevy_mod_raycast::Ray3d;

use crate::game::HolyCam;
use crate::inventory::controller::move_item;
use crate::inventory::controller::CubeRotationAnime;
use crate::inventory::controller::InventoryControllerState;
use crate::inventory::controller::ItemDirection;
use crate::inventory::selection::SelectedItem;

use super::PackedInventoryItem;

#[derive(Component, Debug)]
pub struct Gizmo {
    pub relative: Transform,
    pub item_dir: ItemDirection,
}

pub fn update_gizmo_position(
    mut param_set: ParamSet<(
        Query<(&mut Transform, &Gizmo)>,
        Query<&Transform, With<HolyCam>>,
    )>,
    cube_anime: Res<CubeRotationAnime>,
) {
    let camera_transform = {
        let camera_query = param_set.p1();
        camera_query.single().clone()
    };

    let mut gizmo_pos_query = param_set.p0();
    if cube_anime.enabled {
        cube_anime.start_rotation + cube_anime.anime_time.percent() * cube_anime.end_rotation
    } else {
        cube_anime.end_rotation
    };
    for (mut transform, gizmo) in gizmo_pos_query.iter_mut() {
        let t = camera_transform.translation;
        match gizmo.item_dir {
            ItemDirection::UP
            | ItemDirection::LEFT
            | ItemDirection::DOWN
            | ItemDirection::RIGHT
            | ItemDirection::FORWARD
            | ItemDirection::BACKWARDS => {
                transform.translation = t
                    + camera_transform.forward() * 1.5
                    + camera_transform.right() * 0.8
                    + camera_transform.up() * -0.3;
            }
            ItemDirection::YAW_LEFT
            | ItemDirection::YAW_RIGHT
            | ItemDirection::ROLL_LEFT
            | ItemDirection::ROLL_RIGHT
            | ItemDirection::PITCH_BACKWARDS
            | ItemDirection::PITCH_FORWARD => {
                let gt = gizmo.relative.translation;
                transform.translation = t
                    + camera_transform.forward() * (1.5 + gt.x)
                    + camera_transform.right() * (-0.8 + gt.y)
                    + camera_transform.up() * (-0.3 + gt.z);
            }
        }
        transform.rotation = camera_transform.rotation.mul_quat(gizmo.relative.rotation);
    }
}

pub fn highlight_gizmo(
    mut param_set: ParamSet<(
        Query<(&mut Transform, &Gizmo, &Handle<Mesh>)>,
        Query<(&Camera, &GlobalTransform), With<HolyCam>>,
    )>,
    meshes: Res<Assets<Mesh>>,
    mouse_input: Res<Input<MouseButton>>,
    state: Res<InventoryControllerState>,
    mut query_items: Query<(Entity, &mut PackedInventoryItem)>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    selected: Res<SelectedItem>,
) {
    let cursor_pos = { query_window.single().cursor_position() };
    if let Some(position) = cursor_pos {
        let ray: Ray3d = {
            let camera_param = param_set.p1();
            let (camera, camera_pos) = camera_param.single();
            Ray3d::from_screenspace(position, camera, camera_pos).unwrap()
        };

        let mut gizmo_query = param_set.p0();
        let optional_intersection = {
            let mut found_intersection: bool = false;
            let mut selected_gizmo: Option<&Gizmo> = Option::None;
            for (mut trans, gizmo, mesh) in gizmo_query.iter_mut() {
                if let Some(intersection) = ray_intersection_over_mesh(
                    meshes.get(mesh).unwrap(),
                    &trans.compute_matrix(),
                    &ray,
                    bevy_mod_raycast::Backfaces::Cull,
                ) {
                    trans.scale = Vec3::from((1.1, 1.1, 1.1));
                    if mouse_input.just_pressed(MouseButton::Left) {
                        if !found_intersection {
                            found_intersection = true;
                            selected_gizmo = Some(gizmo);
                        }
                    }
                } else {
                    trans.scale = Vec3::from((1.0, 1.0, 1.0));
                }
            }
            selected_gizmo
        };

        match optional_intersection {
            Some(g) => {
                for mut item in query_items.iter_mut() {
                    match g.item_dir {
                        ItemDirection::UP
                        | ItemDirection::LEFT
                        | ItemDirection::DOWN
                        | ItemDirection::RIGHT
                        | ItemDirection::FORWARD
                        | ItemDirection::BACKWARDS => {
                            if Some(item.0) == selected.selected_entity {
                                move_item(&mut item.1, g.item_dir, state.view_index)
                            }
                        }
                        ItemDirection::YAW_LEFT
                        | ItemDirection::YAW_RIGHT
                        | ItemDirection::ROLL_LEFT
                        | ItemDirection::ROLL_RIGHT
                        | ItemDirection::PITCH_BACKWARDS
                        | ItemDirection::PITCH_FORWARD => {}
                    }
                }
            }
            _ => {}
        };
    }
}
