use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh, GltfNode},
    log,
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::prelude::Collider;

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_gltf_load_event);
    }
}

pub fn load_level(asset_path: &str, commands: &mut Commands, asset_server: &ResMut<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load(asset_path),
        ..default()
    });
}

#[allow(clippy::too_many_arguments)]
fn handle_gltf_load_event(
    mut commands: Commands,
    mut load_events: EventReader<AssetEvent<Gltf>>,
    _mesh_handle_query: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    nodes: Res<Assets<GltfNode>>,
    assets: Res<Assets<Gltf>>,
    asset_server: Res<AssetServer>,
) {
    for event in load_events.iter() {
        if let AssetEvent::Created { handle } = event {
            match asset_server.get_load_state(handle) {
                LoadState::Loaded => {
                    if let Some(scene) = assets.get(handle) {
                        for (name, node_handle) in &scene.named_nodes {
                            if name.to_lowercase().contains("plane")
                                || name.to_lowercase().contains("wall")
                            {
                                dbg!(name);
                                if let (Some(mesh), Some(transform)) = (
                                    get_mesh_from_gltf_node(
                                        node_handle,
                                        &meshes,
                                        &gltf_meshes,
                                        &nodes,
                                    ),
                                    nodes.get(node_handle).map(|node| node.transform),
                                ) {
                                    match get_collider_from_mesh(mesh, &transform) {
                                        Ok(collider) => {
                                            commands.spawn(collider);
                                        }
                                        Err(err) => {
                                            log::error!("{err:?}");
                                        }
                                    }
                                } else {
                                    log::error!(
                                        "Node {name:?} was missing either a mesh or a transform"
                                    );
                                }
                            }
                        }
                        dbg!(scene.named_nodes.keys());
                    }
                }
                LoadState::Failed => {
                    log::error!("gltf failed to load dog");
                }
                _ => {}
            }
        }
    }
}

fn get_mesh_from_gltf_node<'a>(
    node_handle: &Handle<GltfNode>,
    meshes: &'a Res<Assets<Mesh>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
    nodes: &Res<Assets<GltfNode>>,
) -> Option<&'a Mesh> {
    nodes
        .get(node_handle)
        .and_then(|node| node.mesh.as_ref())
        .and_then(|mesh_handle| gltf_meshes.get(mesh_handle))
        .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
        .and_then(|first_primitive| meshes.get(&first_primitive.mesh))
}

// taken from https://github.com/Defernus/bevy_gltf_collider/blob/9f27253e6d2e645c3570bebead34a493e4da1deb/src/mesh_collider.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColliderFromMeshError {
    MissingPositions,
    MissingIndices,
    InvalidIndicesCount(usize),
    InvalidPositionsType(&'static str),
}

fn get_collider_from_mesh(
    mesh: &Mesh,
    transform: &Transform,
) -> Result<Collider, ColliderFromMeshError> {
    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .map_or(Err(ColliderFromMeshError::MissingPositions), Ok)?;

    let indices = mesh
        .indices()
        .map_or(Err(ColliderFromMeshError::MissingIndices), Ok)?;

    let positions = match positions {
        VertexAttributeValues::Float32x3(positions) => positions,
        v => {
            return Err(ColliderFromMeshError::InvalidPositionsType(
                v.enum_variant_name(),
            ));
        }
    };

    let indices: Vec<u32> = match indices {
        Indices::U32(indices) => indices.clone(),
        Indices::U16(indices) => indices.iter().map(|&i| i as u32).collect(),
    };

    if indices.len() % 3 != 0 {
        return Err(ColliderFromMeshError::InvalidIndicesCount(indices.len()));
    }

    let triple_indices = indices.chunks(3).map(|v| [v[0], v[1], v[2]]).collect();
    let vertices = positions
        .iter()
        .map(|v| {
            let p = Vec4::new(v[0], v[1], v[2], 1.0);
            let p_transformed = transform.compute_matrix() * p;
            Vec3::new(
                p_transformed.x / p_transformed.w,
                p_transformed.y / p_transformed.w,
                p_transformed.z / p_transformed.w,
            )
        })
        .collect();

    let collider = Collider::trimesh(vertices, triple_indices);

    Ok(collider)
}
