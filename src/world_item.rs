use crate::collectable::Collectable;
use crate::game_state::GameState;
use bevy::prelude::*;

use crate::inventory::InventoryItem;
use crate::player::combat::PlayerCombatState;

pub const VOXEL_SIZE_IN_WORLD: f32 = 0.1;

#[derive(Component)]
pub struct AttachedToPlayer(bool);

#[derive(Component)]
pub struct WeaponHolder {
    pub current_weapon: Option<(Entity, InventoryItem)>,
}

impl InventoryItem {
    pub fn create_world_entity(
        &self,
        location: Vec3,
        on_player: bool,
        collectable: bool,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        return commands
            .spawn((AttachedToPlayer(on_player), Collectable(collectable)))
            .insert(PbrBundle {
                mesh: meshes.add(self.generate_mesh()),
                material: materials.add(self.color.clone().into()),
                ..default()
            })
            .insert(TransformBundle::from(
                Transform::from_translation(location).with_scale(Vec3::splat(VOXEL_SIZE_IN_WORLD)),
            ))
            .id();
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
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(&Transform, &WeaponHolder, &PlayerCombatState)>,
        Query<(Entity, &mut Transform, &AttachedToPlayer)>,
    )>,
    time: Res<Time>,
) {
    let binding = param_set.p0();
    let player_transform = binding.single().0.clone();
    let entity = binding.single().1.current_weapon.clone().map(|x| x.0);
    let state = binding.single().2.clone();
    drop(binding);
    let mut query = param_set.p1();
    for mut item in query.iter_mut() {
        if !item.2 .0 {
            continue;
        }
        if entity != Some(item.0) {
            commands.entity(item.0).despawn();
            continue;
        }

        item.1.translation = player_transform.translation + player_transform.forward() * 0.5;
        item.1.rotation = player_transform.rotation;
        item.1.rotate_y(180.0f32.to_radians());
        item.1.rotate_y(state.get_weapon_angle(&time));
    }
}
