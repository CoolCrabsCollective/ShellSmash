use crate::game::HolyCam;
use crate::inventory::controller::move_item;
use crate::inventory::controller::ItemDirection;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::render::camera;
use bevy::window::PrimaryWindow;
use bevy_mod_raycast::ray_intersection_over_mesh;
use bevy_mod_raycast::Ray3d;

use crate::inventory::controller::InventoryControllerState;

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
) {
    let camera_transform = {
        let camera_query = param_set.p1();
        camera_query.single().clone()
    };

    let mut gizmo_pos_query = param_set.p0();
    for (mut transform, gizmo) in gizmo_pos_query.iter_mut() {
        let t = camera_transform.translation;
        transform.translation = t
            + camera_transform.forward() * 1.5
            + camera_transform.right() * 0.8
            + camera_transform.up() * -0.3;
        transform.rotation = gizmo.relative.rotation;
        // transform.rotation = camera_transform.rotation.mul_quat(gizmo.relative.rotation);
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
    mut query_voxel: Query<&mut PackedInventoryItem>,
    query_window: Query<&Window, With<PrimaryWindow>>,
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
                            selected_gizmo = Option::Some(gizmo);
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
                for mut item in query_voxel.iter_mut() {
                    move_item(&mut item, g.item_dir, state.view_index)
                }
            }
            _ => {}
        };
    }
}
