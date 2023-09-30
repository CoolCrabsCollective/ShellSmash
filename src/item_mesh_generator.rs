use crate::inventory::InventoryItem;
use bevy::math::vec3;
use bevy::prelude::{Mesh, Vec3};
use bevy::render::mesh::{Indices, PrimitiveTopology};

struct VoxelFace {
    position: Vec3,
    normal: Vec3,
}

impl InventoryItem {
    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut faces: Vec<VoxelFace> = Vec::new();

        let dirs = vec![
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, -1.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            vec3(-1.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, -1.0),
        ];

        for point in &self.local_points {
            'dirs: for dir in &dirs {
                let new_pos = vec3(
                    point.x as f32 + dir.x * 0.5,
                    point.y as f32 + dir.y * 0.5,
                    point.z as f32 + dir.z * 0.5,
                );

                for face in &faces {
                    if face.position == new_pos {
                        continue 'dirs;
                    }
                }

                faces.push(VoxelFace {
                    position: new_pos,
                    normal: dir.clone(),
                });
            }
        }

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        let mut i = 0;
        for face in &faces {
            let orth1 = vec3(face.normal.y, face.normal.z, face.normal.x);
            let orth2 = vec3(face.normal.z, face.normal.x, face.normal.y);
            vertices.push(face.position + orth1 * 0.5 + orth2 * 0.5);
            vertices.push(face.position - orth1 * 0.5 + orth2 * 0.5);
            vertices.push(face.position + orth1 * 0.5 - orth2 * 0.5);
            vertices.push(face.position - orth1 * 0.5 - orth2 * 0.5);

            normals.push(face.normal);
            normals.push(face.normal);
            normals.push(face.normal);
            normals.push(face.normal);

            if face.normal.x + face.normal.y + face.normal.z < 0.0 {
                indices.push(i + 0);
                indices.push(i + 1);
                indices.push(i + 2);

                indices.push(i + 3);
                indices.push(i + 2);
                indices.push(i + 1);
            } else {
                indices.push(i + 2);
                indices.push(i + 1);
                indices.push(i + 0);

                indices.push(i + 1);
                indices.push(i + 2);
                indices.push(i + 3);
            }

            i += 4;
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        mesh.set_indices(Some(Indices::U32(indices)));

        return mesh;
    }
}
