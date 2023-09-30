use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct WallBundle {
    pbr_material: PbrBundle,
    collider: Collider,
    wall: Wall,
}

#[derive(Component)]
pub struct Wall;

impl WallBundle {
    pub fn _from_lengths(
        hx: f32,
        hy: f32,
        hz: f32,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr_material: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(hx * 2.0, hy * 2.0, hz * 2.0))),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_translation(position),
                ..default()
            },
            collider: Collider::cuboid(hx, hy, hz),
            wall: Wall,
        }
    }

    pub fn _from_corners(
        corner_a: Vec3,
        corner_b: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let half_size = (corner_a - corner_b).abs() / 2.0;
        let position = (corner_a + corner_b) / 2.0;
        Self::_from_lengths(
            half_size.x,
            half_size.y,
            half_size.z,
            position,
            meshes,
            materials,
        )
    }
}
