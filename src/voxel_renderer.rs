use bevy::{input::keyboard::KeyboardInput, pbr::wireframe::Wireframe, prelude::*};
use rand::random;

use crate::{
    config::{INVENTORY_GRID_DIMENSIONS, VOXEL_RENDERER_LEFT_RIGHT_CONTROLS},
    game_state::GameState,
    inventory::InventoryData,
    math::deg_to_rad,
};

const GRID_HALF_SIZE: [i32; 3] = [
    INVENTORY_GRID_DIMENSIONS[0] / 2,
    INVENTORY_GRID_DIMENSIONS[1] / 2,
    INVENTORY_GRID_DIMENSIONS[2] / 2,
];

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
    _position: IVec3,
    _color: Color,
}

#[derive(Component)]
struct Voxel(IVec3);

impl Plugin for VoxelRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ManagingInventory), init_voxel_grid);
        app.add_systems(OnExit(GameState::ManagingInventory), kill_voxel_grid);
        app.add_systems(
            Update,
            update_voxels.run_if(in_state(GameState::ManagingInventory)),
        );
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

fn kill_voxel_grid(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<Entity, With<VoxelCoordinateFrame>>,
        Query<Entity, With<Voxel>>,
    )>,
) {
    for entity in param_set.p0().iter() {
        commands.entity(entity).despawn();
    }
    for entity in param_set.p1().iter() {
        commands.entity(entity).despawn();
    }
}

fn init_voxel_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let parent = commands
        .spawn((
            VoxelCoordinateFrame,
            SpatialBundle::from(Transform {
                translation: Vec3::from((500.0, 0.0, 0.0)),
                ..default()
            }),
        ))
        .id();
    for x in 0..INVENTORY_GRID_DIMENSIONS[0] {
        for y in 0..INVENTORY_GRID_DIMENSIONS[1] {
            for z in 0..INVENTORY_GRID_DIMENSIONS[2] {
                let child = commands
                    .spawn((
                        VoxelBundle::new(
                            IVec3::new(
                                x - INVENTORY_GRID_DIMENSIONS[0] / 2,
                                y - INVENTORY_GRID_DIMENSIONS[1] / 2,
                                z - INVENTORY_GRID_DIMENSIONS[2] / 2,
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
    inventory_data_res: Res<InventoryData>,
) {
    for (Voxel(voxel_position), voxel_material_handle) in &voxel_query {
        if let Some(material) = materials.get_mut(voxel_material_handle) {
            material.base_color = Color::rgba(0.0, 0.0, 0.0, 0.0);
            material.alpha_mode = AlphaMode::Blend;

            for (x, x_list) in inventory_data_res.grid.iter().enumerate() {
                for (y, y_list) in x_list.iter().enumerate() {
                    for (z, item_opt) in y_list.iter().enumerate() {
                        if let Some(inventory_item) = item_opt {
                            let location = IVec3::new(x as i32, y as i32, z as i32)
                                - IVec3::new(
                                    GRID_HALF_SIZE[0],
                                    GRID_HALF_SIZE[1],
                                    GRID_HALF_SIZE[2],
                                );
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
}
