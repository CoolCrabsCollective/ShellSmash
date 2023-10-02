use bevy::{prelude::*, window::PrimaryWindow};

use crate::inventory::ItemType::NON_WEAPON;
use crate::inventory::{InventoryItem, ItemTypeId};
use crate::player::combat::PlayerCombatState;
use crate::{game::HolyCam, game_camera_controller, game_state::GameState};

pub struct HealthBarPlugin;

#[derive(Component)]
pub struct UIHeart;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_ui
                .run_if(in_state(GameState::FightingInArena))
                .after(game_camera_controller::set_camera),
        );
    }
}

fn update_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cam_query: Query<&Transform, With<HolyCam>>,
    mut heart_query: Query<
        (
            &mut Transform,
            Entity,
            &UIHeart,
            &mut Handle<StandardMaterial>,
        ),
        Without<HolyCam>,
    >,
    player_query: Query<&PlayerCombatState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let heart = InventoryItem::from((
        (0, 0, 0),
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (1, 0, 0),
            (-1, 0, -1),
            (1, 0, -1),
        ],
        Color::rgba(1.0, 0.1, 0.1, 1.0),
        NON_WEAPON,
        ItemTypeId::Heart,
    ));

    let camera_transform = cam_query.single();
    let ui_entity_position = camera_transform.translation + camera_transform.forward() * 10.0;
    let window = window_query.single();
    let resolution = window.resolution.clone();

    let aspect_ratio_vec = Vec2::new(resolution.width(), resolution.height()).normalize();
    let distance = 1.5;
    let ui_entity_scale = 0.075 * 0.5;
    let mut ui_entity_transform = Transform::default()
        .with_translation(
            ui_entity_position
                + camera_transform.left() * aspect_ratio_vec.x * distance * 0.5
                + camera_transform.up() * aspect_ratio_vec.y * distance,
        )
        .with_scale(Vec3::splat(ui_entity_scale));

    let heart_count = player_query.single().max_hp;
    let healthy_heart_count = player_query.single().current_hp;

    let mut i = 0;
    for mut heart in heart_query.iter_mut() {
        *heart.0 = ui_entity_transform;
        ui_entity_transform.translation += camera_transform.left() * -0.15;
        materials.get_mut(heart.3.as_ref()).unwrap().base_color = if i < healthy_heart_count {
            Color::rgba(1.0, 0.1, 0.1, 1.0)
        } else {
            Color::rgba(0.7, 0.7, 0.7, 0.5)
        };
        i += 1;
    }

    while i < heart_count {
        ui_entity_transform.translation += camera_transform.left() * -0.15;
        let mut heart2 = heart.clone();
        heart2.color = if i < healthy_heart_count {
            Color::rgba(1.0, 0.1, 0.1, 1.0)
        } else {
            Color::rgba(0.7, 0.7, 0.7, 0.5)
        };
        heart2.create_ui_entity(
            ui_entity_transform,
            false,
            false,
            true,
            &mut commands,
            &mut meshes,
            &mut materials,
            None,
        );

        i += 1;
    }
}
