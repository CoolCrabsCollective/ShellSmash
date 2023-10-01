use bevy::{
    asset::{Asset, LoadState},
    log,
    prelude::*,
};

use crate::game_state::GameState;

const JELLY_ASSET_PATH: &str = "jelly.glb#Scene0";

pub struct AssetLoaderPlugin;

// TODO: create pending version
#[derive(Default, Debug, Clone)]
pub struct LoadedSingleModelScene {
    pub scene_handle: Option<Handle<Scene>>,
    pub mesh_handle: Option<Handle<Mesh>>,
    pub material_handle: Option<Handle<StandardMaterial>>,
}

#[derive(Resource, Default)]
pub struct GameAssets {
    jelly: LoadedSingleModelScene,
}

impl GameAssets {
    pub fn are_all_assets_loaded(&self) -> bool {
        self.jelly.is_loaded()
    }

    pub fn jelly(&self) -> LoadedSingleModelScene {
        assert!(self.jelly.is_loaded());
        self.jelly.clone()
    }
}

impl LoadedSingleModelScene {
    pub fn is_loaded(&self) -> bool {
        self.scene_handle.is_some() && self.mesh_handle.is_some() && self.material_handle.is_some()
    }
}

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_all_game_assets, handle_scene_load_event));
        app.add_systems(
            Update,
            handle_scene_load_event.run_if(in_state(GameState::Loading)),
        );
        app.add_systems(
            Update,
            handle_mesh_load_event.run_if(in_state(GameState::Loading)),
        );
        app.insert_resource(GameAssets::default());
    }
}

fn load_all_game_assets(mut asset_server: ResMut<AssetServer>) {
    let _ignored_scene_handle: Handle<Scene> = asset_server.load(JELLY_ASSET_PATH);
}

fn handle_scene_load_event(
    mut load_events: EventReader<AssetEvent<Scene>>,
    mut game_assets: ResMut<GameAssets>,
    mut game_state_updater: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    scenes: Res<Assets<Scene>>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    mesh_handle_query: Query<&Handle<Mesh>>,
    material_handle_query: Query<&Handle<StandardMaterial>>,
) {
    for event in load_events.iter() {
        if let AssetEvent::Created {
            handle: scene_handle,
        } = event
        {
            // Failed
            match asset_server.get_load_state(scene_handle) {
                LoadState::Loaded => {
                    if asset_server
                        .get_handle_path(scene_handle)
                        .expect("Couldn't get asset path?")
                        == JELLY_ASSET_PATH.into()
                    {
                        dbg!(asset_server.get_handle_path(scene_handle));
                        let (mesh_handle, material_handle) = get_first_mesh_material_from_scene(
                            scene_handle.clone(),
                            &scenes,
                            &meshes,
                            &materials,
                            &mesh_handle_query,
                            &material_handle_query,
                        );
                        game_assets.jelly = Some(LoadedSingleModelScene {
                            scene_handle: scene_handle.clone(),
                            mesh_handle,
                            material_handle,
                        });
                    }
                }
                LoadState::Failed => {
                    log::error!("scene failed to load dog");
                }
                _ => {}
            }

            // if handle.
        }
    }

    if game_assets.are_all_assets_loaded() {
        game_state_updater.set(GameState::FightingInArena);
    }
}

fn handle_mesh_load_event(
    mut load_events: EventReader<AssetEvent<Mesh>>,
    mut game_assets: ResMut<GameAssets>,
    mut game_state_updater: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    scenes: Res<Assets<Scene>>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    mesh_handle_query: Query<&Handle<Mesh>>,
    material_handle_query: Query<&Handle<StandardMaterial>>,
) {
    for event in load_events.iter() {
        if let AssetEvent::Created {
            handle: scene_handle,
        } = event
        {
            // Failed
            match asset_server.get_load_state(scene_handle) {
                LoadState::Loaded => {
                    if asset_server
                        .get_handle_path(scene_handle)
                        .expect("Couldn't get asset path?")
                        == JELLY_ASSET_PATH.into()
                    {
                        dbg!(asset_server.get_handle_path(scene_handle));
                        let (mesh_handle, material_handle) = get_first_mesh_material_from_scene(
                            scene_handle.clone(),
                            &scenes,
                            &meshes,
                            &materials,
                            &mesh_handle_query,
                            &material_handle_query,
                        );
                        game_assets.jelly = Some(LoadedSingleModelScene {
                            scene_handle: scene_handle.clone(),
                            mesh_handle,
                            material_handle,
                        });
                    }
                }
                LoadState::Failed => {
                    log::error!("scene failed to load dog");
                }
                _ => {}
            }

            // if handle.
        }
    }

    if game_assets.are_all_assets_loaded() {
        game_state_updater.set(GameState::FightingInArena);
    }
}

fn get_first_mesh_material_from_scene(
    scene_handle: Handle<Scene>,
    scenes: &Res<Assets<Scene>>,
    meshes: &Res<Assets<Mesh>>,
    materials: &Res<Assets<StandardMaterial>>,
    mesh_handle_query: &Query<&Handle<Mesh>>,
    material_handle_query: &Query<&Handle<StandardMaterial>>,
) -> (Handle<Mesh>, Handle<StandardMaterial>) {
    let scene = scenes.get(&scene_handle).unwrap();

    let first_entity = scene
        .world
        .iter_entities()
        .next()
        .expect("No objects found in scene");

    let mesh_handle = mesh_handle_query
        .get(first_entity.id())
        .expect("First object had no mesh")
        .clone();
    let material_handle = material_handle_query
        .get(first_entity.id())
        .expect("First object had no material")
        .clone();

    (mesh_handle, material_handle)
}

fn asset_is_loaded(
    asset: Option<LoadedSingleModelScene>,
    asset_server: &mut ResMut<AssetServer>,
) -> bool {
    asset.is_some() && asset_server.get_load_state(asset.unwrap().scene_handle) == LoadState::Loaded
}

fn bevy_asset_is_loaded<T>(
    asset_handle: Option<Handle<T>>,
    asset_server: &mut ResMut<AssetServer>,
) -> bool
where
    T: Asset,
{
    asset_handle.is_some()
        && asset_server.get_load_state(asset_handle.unwrap()) == LoadState::Loaded
}
