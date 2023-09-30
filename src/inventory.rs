use bevy::prelude::*;

pub struct InventoryItem {
    pub location: IVec3,    // world location
    pub points: Vec<IVec3>, // relative coordinate
}

impl InventoryItem {
    pub(crate) fn spawn_cubes(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
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

    fn intersects(&self, other_location: IVec3) -> bool {
        let relative_location: IVec3 = self.location - other_location;
        for point in &self.points {
            if *point == relative_location {
                return true;
            }
        }
        false
    }
}
