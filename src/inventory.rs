use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryItem {
    pub location: IVec3,    // world location
    pub points: Vec<IVec3>, // relative coordinate, center is the first point
}

impl InventoryItem {
    pub(crate) fn spawn_cubes(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for point in &self.points {
            // cube
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(
                    (self.location.x + point.x) as f32,
                    (self.location.y + point.y) as f32,
                    (self.location.z + point.z) as f32,
                ),
                ..default()
            });
        }
    }

    pub fn _intersects(&self, other_location: IVec3) -> bool {
        let relative_location: IVec3 = self.location - other_location;
        for point in &self.points {
            if *point == relative_location {
                return true;
            }
        }
        false
    }

    pub fn translate(&mut self, translation: IVec3) {
        self.location = translation;
    }

    pub fn _rotate_x(&mut self, ccw: bool) {
        let rot_angle = ((if ccw { 90 } else { -90 }) as f32).to_radians();

        let rot_mat = Mat3::from_rotation_x(rot_angle);
        for p in self.points.iter_mut() {
            let vec3 = Vec3::new(p.x as f32, p.y as f32, p.z as f32);
            let new_p: Vec3 = rot_mat.mul_vec3(vec3);
            p.x = new_p.x as i32;
            p.y = new_p.y as i32;
            p.z = new_p.z as i32;
        }
    }
    pub fn _rotate_y(&mut self) {}
    pub fn _rotate_z(&mut self) {}

    fn _get_center(&self) -> &IVec3 {
        self.points.first().unwrap()
    }
}

impl From<((i32, i32, i32), Vec<(i32, i32, i32)>)> for InventoryItem {
    fn from(value: ((i32, i32, i32), Vec<(i32, i32, i32)>)) -> Self {
        InventoryItem {
            location: value.0.into(),
            points: value.1.iter().map(|tup| (*tup).into()).collect(),
        }
    }
}
