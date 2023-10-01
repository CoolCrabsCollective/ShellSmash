use bevy::prelude::*;

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
