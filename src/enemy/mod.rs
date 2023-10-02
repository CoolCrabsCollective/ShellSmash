use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

use crate::asset_loader::GameAssets;
use crate::config::{
    COLLISION_GROUP_ENEMIES, COLLISION_GROUP_PROJECTILES, COLLISION_GROUP_TERRAIN,
};
use crate::game_state::GameState;
use crate::player::PlayerControllerState;
use crate::projectile::Projectile;

pub const ENEMY_COLLIDER_RADIUS: f32 = 0.25;

pub struct EnemyPlugin;

#[derive(Bundle)]
pub struct EnemyBundle {
    collider: Collider,
    pbr: PbrBundle,
    controller: KinematicCharacterController,
    enemy: Enemy,
}

#[derive(Copy, Clone)]
pub enum EnemyType {
    Jellyfish,
    Urchin,
    Shrimp,
}

#[derive(Component)]
pub struct Enemy {
    enemy_type: EnemyType,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_enemies.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_systems(
            Update,
            detect_enemy_hit.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_systems(
            Update,
            remove_lost_enemies.run_if(in_state(GameState::FightingInArena)),
        );
    }
}

impl EnemyBundle {
    pub fn new(position: Vec3, assets: &Res<GameAssets>, enemy_type: EnemyType) -> Self {
        Self {
            collider: Collider::ball(ENEMY_COLLIDER_RADIUS),
            pbr: PbrBundle {
                mesh: match enemy_type {
                    EnemyType::Jellyfish => assets.jelly(),
                    EnemyType::Urchin => assets.urchin(),
                    EnemyType::Shrimp => assets.shrimp(),
                    _ => assets.urchin(),
                }
                .mesh_handle,
                material: match enemy_type {
                    EnemyType::Jellyfish => assets.jelly(),
                    EnemyType::Urchin => assets.urchin(),
                    EnemyType::Shrimp => assets.shrimp(),
                    _ => assets.urchin(),
                }
                .material_handle,
                transform: Transform::default().with_translation(position),
                ..default()
            },
            controller: KinematicCharacterController {
                // The character offset is set to 0.01.
                offset: CharacterLength::Absolute(0.01),
                up: Vec3::Y,
                // Don’t allow climbing slopes larger than 45 degrees.
                max_slope_climb_angle: 45.0f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30.0f32.to_radians(),
                apply_impulse_to_dynamic_bodies: true,
                filter_groups: Some(CollisionGroups {
                    // memberships: COLLISION_GROUP_ENEMIES,
                    // filters: Group::NONE,
                    // memberships: COLLISION_GROUP_ENEMIES,
                    memberships: COLLISION_GROUP_ENEMIES,
                    filters: COLLISION_GROUP_TERRAIN | COLLISION_GROUP_PROJECTILES,
                }),
                ..default()
            },
            enemy: Enemy { enemy_type },
        }
    }
}

fn move_enemies(
    mut param_set: ParamSet<(
        Query<&Transform, With<PlayerControllerState>>,
        Query<(&mut KinematicCharacterController, &mut Transform), With<Enemy>>,
    )>,
    time: Res<Time>,
) {
    let player_query = param_set.p0();
    let player_position = player_query.single().translation;

    let mut enemy_query = param_set.p1();
    for (mut k_controller, mut transform) in &mut enemy_query {
        // looked kinda cool without normalize tho :eyes:
        let to_player_unit_vector = (player_position - transform.translation).normalize();
        let speed = 4.0;

        let mut current_frame_movement = Vec3::ZERO;
        current_frame_movement.y -= 9.81 * time.delta_seconds();
        current_frame_movement += to_player_unit_vector * speed * time.delta_seconds();

        k_controller.translation = Some(current_frame_movement);
        transform.look_at(player_position, Vec3::Y);
    }
}

fn detect_enemy_hit(
    mut commands: Commands,
    enemy_controller_output_query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        With<Enemy>,
    >,
    projectile_entity_query: Query<(Entity, &Projectile)>,
) {
    for (enemy_entity, enemy_controller) in &enemy_controller_output_query {
        for collision in &enemy_controller.collisions {
            if projectile_entity_query.contains(collision.entity) {
                commands.entity(enemy_entity).despawn();
                commands.entity(collision.entity).despawn();
            }
        }
    }
}

fn remove_lost_enemies(
    mut commands: Commands,
    enemy_entity_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for enemy in enemy_entity_query.iter() {
        if enemy.1.translation.y < -5.0 {
            commands.entity(enemy.0).despawn();
        }
    }
}
