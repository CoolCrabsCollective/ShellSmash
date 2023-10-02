use bevy::{
    asset::{LoadState, SourcePathId},
    gltf::{Gltf, GltfMesh},
    log,
    prelude::*,
};

use crate::game_state::GameState;

const JELLY_ASSET_PATH: &str = "jelly.glb#Scene0";
const ARROW_STRAIGHT_ASSET_PATH: &str = "arrow_straight.glb#Scene0";
const ARROW_NOT_STRAIGHT_ASSET_PATH: &str = "arrow_rotated.glb#Scene0";

pub struct AssetLoaderPlugin;

#[derive(Debug, Clone)]
pub struct LoadedSingleModelScene {
    pub scene_handle: Handle<Gltf>,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
    // pub transform: Transform,
}

#[derive(Resource, Default)]
pub struct GameAssets {
    jelly_scene_handle: Handle<Gltf>,
    jelly: Option<LoadedSingleModelScene>,

    arrow_straight_scene_handle: Handle<Gltf>,
    arrow_straight: Option<LoadedSingleModelScene>,

    arrow_rotated_scene_handle: Handle<Gltf>,
    arrow_rotated: Option<LoadedSingleModelScene>,
}

impl GameAssets {
    pub fn are_all_assets_loaded(&self) -> bool {
        self.jelly.is_some() && self.arrow_straight.is_some()
    }

    pub fn jelly(&self) -> LoadedSingleModelScene {
        self.jelly.clone().unwrap()
    }

    pub fn arrow_straight(&self) -> LoadedSingleModelScene {
        self.arrow_straight.clone().unwrap()
    }
    pub fn arrow_rotated(&self) -> LoadedSingleModelScene {
        self.arrow_rotated.clone().unwrap()
    }
}

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_all_game_assets, handle_scene_load_event));
        app.add_systems(Update, handle_scene_load_event);
        app.insert_resource(GameAssets::default());
    }
}

fn load_all_game_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAssets>) {
    game_assets.jelly_scene_handle = asset_server.load(JELLY_ASSET_PATH);
    game_assets.arrow_straight_scene_handle = asset_server.load(ARROW_STRAIGHT_ASSET_PATH);
    game_assets.arrow_rotated_scene_handle = asset_server.load(ARROW_NOT_STRAIGHT_ASSET_PATH);
}

fn handle_scene_load_event(
    mut load_events: EventReader<AssetEvent<Gltf>>,
    mut game_assets: ResMut<GameAssets>,
    game_state: Res<State<GameState>>,
    mut game_state_updater: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    scenes: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
) {
    for event in load_events.iter() {
        if let AssetEvent::Created {
            handle: scene_handle,
        } = event
        {
            match asset_server.get_load_state(scene_handle) {
                LoadState::Loaded => {
                    if is_same_asset(scene_handle.clone(), game_assets.jelly_scene_handle.clone()) {
                        let (mesh_handle, material_handle) = get_first_mesh_material_from_scene(
                            scene_handle.clone(),
                            &scenes,
                            &gltf_meshes,
                        );
                        game_assets.jelly = Some(LoadedSingleModelScene {
                            scene_handle: scene_handle.clone(),
                            mesh_handle,
                            material_handle,
                        });
                    }

                    if is_same_asset(
                        scene_handle.clone(),
                        game_assets.arrow_straight_scene_handle.clone(),
                    ) {
                        let (mesh_handle, material_handle) = get_first_mesh_material_from_scene(
                            scene_handle.clone(),
                            &scenes,
                            &gltf_meshes,
                        );
                        game_assets.arrow_straight = Some(LoadedSingleModelScene {
                            scene_handle: scene_handle.clone(),
                            mesh_handle,
                            material_handle,
                        });
                    }
                    if is_same_asset(
                        scene_handle.clone(),
                        game_assets.arrow_rotated_scene_handle.clone(),
                    ) {
                        let (mesh_handle, material_handle) = get_first_mesh_material_from_scene(
                            scene_handle.clone(),
                            &scenes,
                            &gltf_meshes,
                        );
                        game_assets.arrow_rotated = Some(LoadedSingleModelScene {
                            scene_handle: scene_handle.clone(),
                            mesh_handle,
                            material_handle,
                        });
                    }
                }
                LoadState::Failed => {
                    log::error!(
                        "scene failed to load dog: asset path = {:?}",
                        asset_server.get_handle_path(scene_handle)
                    );
                }
                _ => {}
            }
        }
    }
}

fn is_same_asset(asset_handle_a: Handle<Gltf>, asset_handle_b: Handle<Gltf>) -> bool {
    match (
        get_source_path_id(asset_handle_a),
        get_source_path_id(asset_handle_b),
    ) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

fn get_source_path_id(asset: Handle<Gltf>) -> Option<SourcePathId> {
    match asset.id() {
        bevy::asset::HandleId::Id(_, _) => None,
        bevy::asset::HandleId::AssetPathId(asset_path_id) => Some(asset_path_id.source_path_id()),
    }
}

fn get_first_mesh_material_from_scene(
    scene_handle: Handle<Gltf>,
    scenes: &Res<Assets<Gltf>>,
    gltf_meshes: &Res<Assets<GltfMesh>>,
) -> (Handle<Mesh>, Handle<StandardMaterial>) {
    let scene = scenes.get(&scene_handle).unwrap();

    let mesh_handle = gltf_meshes
        .get(
            scene
                .meshes
                .get(0)
                .expect("Scene must have at least one mesh"),
        )
        .unwrap()
        .primitives[0]
        .mesh
        .clone();

    let material_handle = gltf_meshes
        .get(
            scene
                .meshes
                .get(0)
                .expect("Scene must have at least one mesh"),
        )
        .unwrap()
        .primitives[0]
        .material
        .clone()
        .expect("Mesh must have a material");

    (mesh_handle, material_handle)
}
