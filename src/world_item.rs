use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::inventory::InventoryItem;

#[derive(Component)]
pub struct AttachedToPlayer(bool);

impl InventoryItem {
    pub fn create_world_entity(
        &self,
        location: Vec3,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands
            .spawn(AttachedToPlayer(false))
            .insert(PbrBundle {
                mesh: meshes.add(self.generate_mesh()),
                material: materials.add(self.color.clone().into()),
                ..default()
            })
            .insert(TransformBundle::from(
                Transform::from_translation(location).with_scale(Vec3::splat(0.1)),
            ));
    }
}
