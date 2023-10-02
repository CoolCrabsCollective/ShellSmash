use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::collectable::Collectable;
use crate::game_state::GameState;
use crate::inventory::ItemType::MELEE_WEAPON;
use crate::inventory::{Inventory, InventoryItem};
use crate::player::combat::PlayerCombatState;

pub const VOXEL_SIZE_IN_WORLD: f32 = 0.2;

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
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        return commands
            .spawn((
                AttachedToPlayer(on_player),
                Collectable(collectable),
                self.clone(),
            ))
            .insert(PbrBundle {
                mesh: meshes.add(self.generate_mesh(true)),
                material: materials.add(self.color.clone().into()),
                ..default()
            })
            .insert(TransformBundle::from(
                Transform::from_translation(location).with_scale(Vec3::splat(VOXEL_SIZE_IN_WORLD)),
            ))
            .id();
    }

    pub fn create_world_entity_but_given_the_freedom_to_pass_your_own_transform_and_collider_like_it_always_should_have_been__god_bless_america_ok_boomer(
        &self,
        transform: Transform,
        on_player: bool,
        collectable: bool,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        collider: Option<Collider>,
    ) -> Entity {
        let mut e_commands = commands.spawn((
            AttachedToPlayer(on_player),
            Collectable(collectable),
            self.clone(),
        ));

        e_commands.insert(PbrBundle {
            mesh: meshes.add(self.generate_mesh(true)),
            material: materials.add(self.color.clone().into()),
            transform,
            ..default()
        });

        if let Some(collider) = collider {
            e_commands.insert(collider);
        }

        return e_commands.id();
    }
}

pub struct ItemAttachmentPlugin;

impl Plugin for ItemAttachmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            item_attachment_update.run_if(in_state(GameState::FightingInArena)),
        );

        app.add_systems(OnEnter(GameState::FightingInArena), equip_update);
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
    let current_weapon = binding
        .single()
        .1
        .current_weapon
        .clone()
        .map(|x| x.1)
        .clone();
    let state = binding.single().2.clone();
    drop(binding);
    let mut query = param_set.p1();
    for mut item in query.iter_mut() {
        if !item.2 .0 {
            // dbg!("not attached?");
            continue;
        }
        if entity != Some(item.0) {
            // dbg!("get fked", entity, item.0);
            commands.entity(item.0).despawn();
            continue;
        }

        // dbg!("ok wtf");

        item.1.translation = player_transform.translation + player_transform.forward() * 0.5;
        item.1.rotation = player_transform.rotation;
        item.1.rotate_y(180.0f32.to_radians());
        if current_weapon.clone().unwrap().item_type == MELEE_WEAPON {
            item.1.rotate_y(state.get_weapon_angle(&time));
        }
    }
}

fn equip_update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_query: Query<(&mut WeaponHolder, &Transform, &PlayerCombatState)>,
    inventory_query: Res<Inventory>,
) {
    let (mut player_weapon, player_transform, _) = player_query.single_mut();

    if !player_weapon.current_weapon.is_none() {
        let player_weapon_id = player_weapon.current_weapon.clone().unwrap().1.item_type_id;

        let mut found_item = false;

        for item in &inventory_query.content {
            if item.item_type_id == player_weapon_id {
                found_item = true;
                break;
            }
        }

        if !found_item {
            dbg!("Deleting current weapon because no longer in inventory");
            player_weapon.current_weapon = None;
        }
    }

    if player_weapon.current_weapon.is_none() && inventory_query.content.len() > 0 {
        let item = inventory_query.content[0].clone();
        let entity = item.create_world_entity(
            player_transform.translation,
            true,
            false,
            &mut commands,
            &mut meshes,
            &mut materials,
        );

        dbg!("adding current weapon!");
        player_weapon.current_weapon = Some((entity, item));
    }
}
