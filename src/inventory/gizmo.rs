use bevy::prelude::*;
use bevy::render::camera;
use bevy::window::PrimaryWindow;
use bevy_mod_raycast::ray_intersection_over_mesh;
use bevy_mod_raycast::Ray3d;

#[derive(Component, Debug)]
pub struct Gizmo {
    pub relative: Transform,
}

pub fn update_gizmo_position(
    mut param_set: ParamSet<(
        Query<(&mut Transform, &Gizmo)>,
        Query<&Transform, With<Camera>>,
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
        transform.rotation = camera_transform.rotation.mul_quat(gizmo.relative.rotation);
    }
}

pub fn highlight_gizmo(
    mut param_set: ParamSet<(
        Query<&Window, With<PrimaryWindow>>,
        Query<(&Transform, &Gizmo, &Handle<Mesh>)>,
        Query<(&Camera, &GlobalTransform)>,
    )>,
    meshes: Res<Assets<Mesh>>,
) {
    let cursor_pos = {
        let windows = param_set.p0();
        windows.single().cursor_position()
    };
    if let Some(position) = cursor_pos {
        let ray: Ray3d = {
            let camera_param = param_set.p2();
            let (camera, camera_pos) = camera_param.single();
            Ray3d::from_screenspace(position, camera, camera_pos).unwrap()
        };

        let mut gizmo_query = param_set.p1();
        for (trans, gizmo, mesh) in gizmo_query.iter_mut() {
            dbg!(mesh);
            if let Some(intersection) = ray_intersection_over_mesh(
                meshes.get(mesh).unwrap(),
                &trans.compute_matrix(),
                &ray,
                bevy_mod_raycast::Backfaces::Cull,
            ) {
                println!("YOO");
            }
        }
    }
}
