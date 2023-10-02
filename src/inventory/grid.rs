use bevy::math::vec2;
use bevy::prelude::*;

pub struct GridDisplayPlugin;

impl Plugin for GridDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct Grid;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("grid.png");
    let texture_handle_selected = asset_server.load("grid_selected.png");

    let grid_size = 7;

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(make_plane_mesh(51)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                emissive: Color::WHITE,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(500.0, -0.5 + 0.01, 0.0)),
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(make_plane_mesh(grid_size)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_selected.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(500.0, 0.5 + 0.01, 0.0)),
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(make_plane_mesh(grid_size)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_selected.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(500.0, -0.5 + 0.01, 0.0)),
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(vec2(grid_size as f32, 2.0)))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_selected.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(500.0, -0.5, 3.5)),
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(vec2(grid_size as f32, 2.0)))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_selected.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(500.0, -0.5, -3.5)),
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(vec2(grid_size as f32, 2.0)))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_selected.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(503.5, -0.5, 0.0))
                .with_rotation(Quat::from_rotation_y(-90.0f32.to_radians())),
            ..default()
        })
        .insert(Grid);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(vec2(grid_size as f32, 2.0)))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle_selected.clone()),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(496.5, -0.5, 0.0))
                .with_rotation(Quat::from_rotation_y(90.0f32.to_radians())),
            ..default()
        })
        .insert(Grid);
}

fn make_plane_mesh(size: i32) -> Mesh {
    let mut mesh: Mesh = Mesh::from(shape::Plane::from_size(size as f32));

    mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 0.0],
            [0.0, size as f32],
            [size as f32, 0.0],
            [size as f32, size as f32],
        ],
    );

    return mesh;
}
