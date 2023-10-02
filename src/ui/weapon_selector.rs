use bevy::{log, prelude::*, window::PrimaryWindow};

use crate::inventory::{Inventory, InventoryItem};
use crate::{
    game::HolyCam,
    game_camera_controller,
    game_state::GameState,
    inventory::ItemType::NON_WEAPON,
    player::{combat::PlayerCombatState, PlayerControllerState},
    world_item::WeaponHolder,
};

pub struct WeaponSelectorPlugin;

#[derive(Resource)]
pub struct NextWeapon {
    value: Option<(Entity, InventoryItem)>,
}

impl Plugin for WeaponSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_next_weapon
                .run_if(in_state(GameState::FightingInArena))
                .after(game_camera_controller::set_camera),
        );

        app.add_systems(
            Update,
            highlight_item_on_hover
                .run_if(in_state(GameState::FightingInArena))
                .after(update_next_weapon),
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
    let new_next_weapon = query_next_weapon(&selected_weapon_query.single(), &inventory);

    if new_next_weapon.is_none() {
        if let Some((entity, _)) = next_weapon.value.as_mut() {
            commands.entity(*entity).despawn();
        }

        next_weapon.value = None;
        return;
    }

    let new_next_weapon = new_next_weapon.unwrap();

    let camera_transform_query = param_set.p2();
    let camera_transform = camera_transform_query.single();
    let ui_entity_position = camera_transform.translation + camera_transform.forward() * 10.0;
    let window = window_query.single();
    // dbg!(window.cursor.);
    let resolution = window.resolution.clone();
    // dbg!(
    //     resolution.width(),
    //     resolution.height(),
    //     window.cursor_position()
    // );
    // let aspect_ratio = resolution.width() / resolution.height();
    let aspect_ratio_vec = Vec2::new(resolution.width(), resolution.height()).normalize();
    let distance = 1.5;
    let ui_entity_scale = 0.075 * 0.5;
    let ui_entity_transform = Transform::default()
        .with_translation(
            ui_entity_position
                + camera_transform.left() * aspect_ratio_vec.x * distance
                + camera_transform.up() * aspect_ratio_vec.y * distance,
        )
        .with_scale(Vec3::splat(ui_entity_scale));

    if next_weapon.value.clone().map(|(_, item)| item.item_type_id)
        == Some(new_next_weapon.item_type_id)
    {
        let mut mesh_material_query = param_set.p0();
        let mut existing_ui_entity = mesh_material_query
            .get_mut(next_weapon.value.as_mut().unwrap().0)
            .unwrap();
        (*existing_ui_entity.0) = ui_entity_transform;
        return;
    }

    let mesh_handle = meshes.add(new_next_weapon.generate_mesh(true));
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
            let entity = new_next_weapon.create_ui_entity(
                ui_entity_transform,
                false,
                false,
                false,
                &mut commands,
                &mut meshes,
                &mut materials,
                None,
                // Some(Collider::ball(3.0)),
            );

            (entity, new_next_weapon)
        }
    };

    next_weapon.value = Some(new_value);
}

fn query_next_weapon(
    weapon_holder: &WeaponHolder,
    inventory: &Res<Inventory>,
) -> Option<InventoryItem> {
    if inventory.content.len() < 2 {
        return None;
    }

    if inventory
        .content
        .iter()
        .all(|item| item.item_type == NON_WEAPON)
    {
        return None;
    }

    let selected_weapon = weapon_holder.current_weapon.clone();

    if selected_weapon.is_none() {
        return None;
    }

    let (_, selected_weapon) = selected_weapon.unwrap();
    let selected_weapon_index = inventory
        .content
        .iter()
        .enumerate()
        .find_map(|(index, item)| {
            (item.item_type_id == selected_weapon.item_type_id).then(|| index)
        });
    let selected_weapon_index = selected_weapon_index.unwrap();
    let mut next_weapon_index = (selected_weapon_index + 1) % inventory.content.len();

    while inventory.content[next_weapon_index].item_type == NON_WEAPON {
        next_weapon_index = (next_weapon_index + 1) % inventory.content.len();
    }

    Some(inventory.content[next_weapon_index].clone())
}

fn highlight_item_on_hover(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transform_query: Query<&mut Transform>,
    mut player_query: Query<(Entity, &mut WeaponHolder, &PlayerCombatState)>,
    next_weapon: Res<NextWeapon>,
    inventory: Res<Inventory>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<Input<MouseButton>>,
) {
    let window = window_query.single();

    let cursor_position = window.cursor_position();
    if cursor_position.is_none() {
        return;
    }
    let cursor_position = cursor_position.unwrap();
    let window_resolution = window.resolution.clone();
    let aspect_ratio = window_resolution.width() / window_resolution.height();

    let cursor_pos_percent = Vec2::new(
        cursor_position.x / window_resolution.width(),
        cursor_position.y / window_resolution.height(),
    );

    if next_weapon.value.is_none() {
        return;
    }
    let (next_weapon_entity, _) = next_weapon.value.as_ref().unwrap().clone();

    if cursor_pos_percent.x < 0.15 && cursor_pos_percent.y < 0.15 * aspect_ratio {
        if let Ok(mut transform) = transform_query.get_mut(next_weapon_entity) {
            transform.scale *= if mouse_button.pressed(MouseButton::Left) {
                1.0
            } else {
                1.1
            };

            if mouse_button.just_pressed(MouseButton::Left) {
                let (player_entity, mut player_weapon, _) = player_query.single_mut();

                if let Some(next_weapon) = query_next_weapon(&player_weapon, &inventory) {
                    let player_transform = transform_query.get(player_entity).unwrap();

                    // despawn current weapon
                    if let Some((entity, _)) = player_weapon.current_weapon {
                        commands.entity(entity).despawn();
                    }

                    let entity = next_weapon.create_world_entity(
                        player_transform.translation,
                        true,
                        false,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );

                    player_weapon.current_weapon = Some((entity, next_weapon));

                    log::info!("Switching to next weapon");
                }
            }
        }
    }
}
