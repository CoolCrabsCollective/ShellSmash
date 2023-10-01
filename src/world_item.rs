use crate::game_state::GameState;
use bevy::prelude::*;

use crate::inventory::InventoryItem;
use crate::player::PlayerControllerState;

pub const VOXEL_SIZE_IN_WORLD: f32 = 0.1;

#[derive(Component)]
pub struct AttachedToPlayer(bool);

#[derive(Component)]
pub struct Collectable(bool);

impl InventoryItem {
    pub fn create_world_entity(
        &self,
        location: Vec3,
        on_player: bool,
        collectable: bool,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands
            .spawn((AttachedToPlayer(on_player), Collectable(collectable)))
            .insert(PbrBundle {
                mesh: meshes.add(self.generate_mesh()),
                material: materials.add(self.color.clone().into()),
                ..default()
            })
            .insert(TransformBundle::from(
                Transform::from_translation(location).with_scale(Vec3::splat(VOXEL_SIZE_IN_WORLD)),
            ));
    }
}

pub struct ItemAttachmentPlugin;

impl Plugin for ItemAttachmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            item_attachment_update.run_if(in_state(GameState::FightingInArena)),
        );
    }
}

pub fn item_attachment_update(
    mut param_set: ParamSet<(
        Query<&Transform, With<PlayerControllerState>>,
        Query<(&mut Transform, &AttachedToPlayer)>,
    )>,
) {
    let player_transform = param_set.p0().single().clone();
    let mut query = param_set.p1();
    for mut item in query.iter_mut() {
        if !item.1 .0 {
            continue;
        }
        item.0.translation = player_transform.translation + player_transform.forward() * 0.5;
        item.0.rotation = player_transform.rotation;
        item.0.rotate_y(180.0f32.to_radians());
    }
}
