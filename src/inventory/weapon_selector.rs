use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::HolyCam, game_state::GameState, inventory::ItemType::NON_WEAPON,
    player::PlayerControllerState, world_item::WeaponHolder,
};

use super::{Inventory, InventoryItem};

pub struct WeaponSelectorPlugin;

#[derive(Resource)]
pub struct NextWeapon {
    value: Option<(Entity, InventoryItem)>,
}

impl Plugin for WeaponSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_next_weapon.run_if(in_state(GameState::FightingInArena)),
        );

        app.insert_resource(NextWeapon { value: None });
    }
}

fn update_next_weapon(
    mut commands: Commands,
    mut next_weapon: ResMut<NextWeapon>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut param_set: ParamSet<(
        Query<(
            &mut Transform,
            &mut Handle<Mesh>,
            &mut Handle<StandardMaterial>,
        )>,
        Query<&Transform, With<PlayerControllerState>>,
        Query<&Transform, With<HolyCam>>,
    )>,
    inventory: Res<Inventory>,
    selected_weapon_query: Query<&WeaponHolder, &PlayerControllerState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut remove_next_weapon = || {
        if let Some((entity, _)) = next_weapon.value.as_mut() {
            commands.entity(*entity).despawn();
        }

        next_weapon.value = None;
    };

    if inventory.content.len() < 2 {
        remove_next_weapon();
        return;
    }

    let selected_weapon = selected_weapon_query.single().current_weapon.clone();

    if selected_weapon.is_none() {
        remove_next_weapon();
        return;
    }

    let (_, selected_weapon) = selected_weapon.unwrap();

    let selected_weapon_index = inventory
        .content
        .iter()
        .enumerate()
        .find_map(|(index, item)| {
            (item.item_type_id == selected_weapon.item_type_id).then(|| index)
        });

    if selected_weapon_index.is_none() {
        // dbg!(&inventory.content, selected_weapon);

        remove_next_weapon();
        return;
    }

    let camera_transform_query = param_set.p2();
    let camera_transform = camera_transform_query.single();
    let ui_entity_position = camera_transform.translation + camera_transform.forward() * 10.0;
    let resolution = window_query.single().resolution.clone();
    let aspect_ratio = resolution.width() / resolution.height();
    let aspect_ratio_vec = Vec2::new(resolution.width(), resolution.height()).normalize();
    let distance = 1.5;
    let ui_entity_transform = Transform::default()
        .with_translation(
            ui_entity_position
                + camera_transform.left() * aspect_ratio_vec.x * distance
                + camera_transform.up() * aspect_ratio_vec.y * distance,
        )
        .with_scale(Vec3::new(0.075, 0.075, 0.075));

    let selected_weapon_index = selected_weapon_index.unwrap();
    let mut next_weapon_index = (selected_weapon_index + 1) % inventory.content.len();

    while inventory.content[next_weapon_index].item_type == NON_WEAPON {
        next_weapon_index = (next_weapon_index + 1) % inventory.content.len();
    }

    if next_weapon.value.clone().map(|(_, item)| item.item_type_id)
        == Some(inventory.content[next_weapon_index].clone().item_type_id)
    {
        let mut mesh_material_query = param_set.p0();
        let mut existing_ui_entity = mesh_material_query
            .get_mut(next_weapon.value.as_mut().unwrap().0)
            .unwrap();
        (*existing_ui_entity.0) = ui_entity_transform;
        return;
    }

    let new_next_weapon = inventory.content[next_weapon_index].clone();
    let mesh_handle = meshes.add(new_next_weapon.generate_mesh());
    let material_handle = materials.add(StandardMaterial::from(new_next_weapon.color));

    let new_value = match next_weapon.value {
        Some((entity, _)) => {
            let mut mesh_material_query = param_set.p0();
            let mut existing_ui_entity = mesh_material_query.get_mut(entity).unwrap();
            (*existing_ui_entity.0) = ui_entity_transform;
            (*existing_ui_entity.1) = mesh_handle;
            (*existing_ui_entity.2) = material_handle;

            (entity, new_next_weapon)
        }
        None => {
            let entity = new_next_weapon.create_world_entity_but_given_the_freedom_to_pass_your_own_scale_like_it_always_should_have_been__god_bless_america(
                ui_entity_transform,
                false,
                false,
                &mut commands,
                &mut meshes,
                &mut materials,
            );

            (entity, new_next_weapon)
        }
    };

    next_weapon.value = Some(new_value);
}
