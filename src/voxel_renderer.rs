use bevy::{pbr::wireframe::Wireframe, prelude::*};

const GRID_DIMS: [i32; 3] = [5, 5, 5];

pub struct VoxelRendererPlugin;

#[derive(Component)]
pub struct VoxelCoordinateFrame;

#[derive(Bundle)]
struct VoxelBundle {
    pbr_material: PbrBundle,
    position: VoxelPosition,
}

#[derive(Component)]
struct VoxelPosition(IVec3);

impl Plugin for VoxelRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_voxel_grid);
        // app.add_systems(Startup, init_voxel_grid);
        // app.add_systems(Update, (process_inputs, update_state, set_camera));
        // app.insert_resource(VoxelGridBundle::new());
    }
}

impl VoxelBundle {
    pub fn new(
        position: IVec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
        Self {
            pbr_material: PbrBundle {
                mesh,
                material: materials.add(StandardMaterial::from(Color::rgb(0.3, 0.5, 0.3))),
                transform: Transform::from_translation(Vec3::new(
                    position.x as f32,
                    position.y as f32,
                    position.z as f32,
                )),
                ..default()
            },
            position: VoxelPosition(position),
        }
    }
}

fn init_voxel_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let parent = commands
        .spawn((VoxelCoordinateFrame, SpatialBundle::default()))
        .id();
    for x in 0..GRID_DIMS[0] {
        for y in 0..GRID_DIMS[1] {
            for z in 0..GRID_DIMS[2] {
                let child = commands
                    .spawn((
                        VoxelBundle::new(IVec3::new(x, y, z), &mut meshes, &mut materials),
                        Wireframe,
                    ))
                    .id();

                commands.entity(parent).add_child(child);
            }
        }
    }
}
