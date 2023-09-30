use bevy::{input::keyboard::KeyboardInput, pbr::wireframe::Wireframe, prelude::*, utils::HashSet};
use rand::random;

use crate::{inventory::InventoryData, math::deg_to_rad};

const LEFT_RIGHT: bool = false;
const GRID_DIMS: [i32; 3] = [5, 5, 5];

pub struct VoxelRendererPlugin;

#[derive(Component)]
pub struct VoxelCoordinateFrame;

#[derive(Bundle)]
struct VoxelBundle {
    pbr_material: PbrBundle,
    voxel: Voxel,
}

#[derive(Debug, Component)]
struct VoxelData {
    position: IVec3,
    color: Color,
}

#[derive(Component)]
struct Voxel(IVec3);

impl Plugin for VoxelRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_voxel_grid);
        app.add_systems(Update, (process_inputs, update_voxels_2));
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
                material: materials.add(StandardMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
                transform: Transform::from_translation(Vec3::new(
                    position.x as f32,
                    position.y as f32,
                    position.z as f32,
                )),
                ..default()
            },
            voxel: Voxel(position),
        }
    }
}

fn process_inputs(
    mut commands: Commands,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<VoxelCoordinateFrame>>,
) {
    let mut coordinate_frame_transform = query.single_mut();

    for event in keyboard_input_events.iter() {
        if event.state.is_pressed() {
            match event.key_code {
                Some(KeyCode::Up) => {
                    coordinate_frame_transform.scale += Vec3::new(0.1, 0.1, 0.1);
                }
                Some(KeyCode::Down) => {
                    coordinate_frame_transform.scale -= Vec3::new(0.1, 0.1, 0.1);
                }
                Some(KeyCode::Left) => {
                    if LEFT_RIGHT {
                        coordinate_frame_transform.rotation *=
                            Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), deg_to_rad(15.0))
                    }
                }
                Some(KeyCode::Right) => {
                    if LEFT_RIGHT {
                        coordinate_frame_transform.rotation *=
                            Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), deg_to_rad(-15.0))
                    }
                }
                Some(KeyCode::R) => {
                    let new_voxel = VoxelData {
                        position: IVec3::new(
                            ((random::<f32>() - 0.5) * GRID_DIMS[0] as f32) as i32,
                            ((random::<f32>() - 0.5) * GRID_DIMS[1] as f32) as i32,
                            ((random::<f32>() - 0.5) * GRID_DIMS[2] as f32) as i32,
                        ),
                        color: Color::rgba(random(), random(), random(), random()),
                    };
                    println!("Spawning new voxel data: {new_voxel:?}");
                    commands.spawn(new_voxel);
                }
                _ => {}
            }
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
                        VoxelBundle::new(
                            IVec3::new(
                                x - GRID_DIMS[0] / 2,
                                y - GRID_DIMS[1] / 2,
                                z - GRID_DIMS[2] / 2,
                            ),
                            &mut meshes,
                            &mut materials,
                        ),
                        Wireframe,
                    ))
                    .id();

                commands.entity(parent).add_child(child);
            }
        }
    }
}

fn update_voxels(
    mut materials: ResMut<Assets<StandardMaterial>>,
    voxel_query: Query<(&Voxel, &Handle<StandardMaterial>)>,
    voxel_data_query: Query<&VoxelData>,
) {
    for (Voxel(voxel_position), voxel_material_handle) in &voxel_query {
        if let Some(material) = materials.get_mut(voxel_material_handle) {
            material.base_color = Color::rgba(0.0, 0.0, 0.0, 0.0);
            material.alpha_mode = AlphaMode::Blend;

            for voxel_data in &voxel_data_query {
                if *voxel_position == voxel_data.position {
                    material.base_color = voxel_data.color;
                    material.alpha_mode = if voxel_data.color.a() < 1.0 {
                        AlphaMode::Blend
                    } else {
                        AlphaMode::Opaque
                    };
                }
            }
        }
    }
}

fn update_voxels_2(
    mut materials: ResMut<Assets<StandardMaterial>>,
    voxel_query: Query<(&Voxel, &Handle<StandardMaterial>)>,
    inventory_data_res: Res<InventoryData>,
) {
    // let mut count = 0;
    let mut locations: HashSet<IVec3> = HashSet::new();
    for (Voxel(voxel_position), voxel_material_handle) in &voxel_query {
        if let Some(material) = materials.get_mut(voxel_material_handle) {
            material.base_color = Color::rgba(0.0, 0.0, 0.0, 0.0);
            material.alpha_mode = AlphaMode::Blend;

            for (x, x_list) in inventory_data_res.grid.iter().enumerate() {
                for (y, y_list) in x_list.iter().enumerate() {
                    for (z, item_opt) in y_list.iter().enumerate() {
                        if let Some(inventory_item) = item_opt {
                            let location =
                                IVec3::new(x as i32, y as i32, z as i32) - IVec3::new(2, 2, 2);
                            locations.insert(location);
                            // dbg!(location);
                            if *voxel_position == location {
                                material.base_color = inventory_item.color;
                                material.alpha_mode = if inventory_item.color.a() < 1.0 {
                                    AlphaMode::Blend
                                } else {
                                    AlphaMode::Opaque
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    // dbg!(locations);
}
