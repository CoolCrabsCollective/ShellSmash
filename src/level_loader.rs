use bevy::{prelude::*, utils::HashSet};

pub struct LevelLoaderPlugin;

#[derive(Resource)]
struct LevelLoaderState {
    pending_scenes: HashSet<String>,
    loaded_scenes: HashSet<String>,
}

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_load_event);
    }
}

pub fn load_level(
    asset_path: &str,
    asset_server: Res<AssetServer>,
    // commands: &mut Commands,
    // meshes: &mut ResMut<Assets<Mesh>>,
    // materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let yo: Handle<Scene> = asset_server.load(asset_path);
}

fn handle_load_event(mut load_events: EventReader<AssetEvent<Scene>>) {
    for event in load_events.iter() {
        if let AssetEvent::Created { handle } = event {
            // if handle.
        }
    }
}
