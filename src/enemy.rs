use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;

use crate::game_state::GameState;
use crate::player::PlayerControllerState;

pub struct EnemyPlugin;

#[derive(Bundle)]
pub struct EnemyBundle {
    collider: Collider,
    scene: SceneBundle,
    controller: KinematicCharacterController,
    enemy: Enemy,
}

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_enemies.run_if(in_state(GameState::FightingInArena)),
        );
    }
}

impl EnemyBundle {
    pub fn new(asset_server: &mut ResMut<AssetServer>, position: Vec3) -> Self {
        Self {
            collider: Collider::capsule_y(0.3, 0.25),
            scene: SceneBundle {
                scene: asset_server.load("jelly.glb#Scene0"),
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
                ..default()
            },
            enemy: Enemy,
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
        let to_player_unit_vector = player_position - transform.translation;
        let speed = 1.0;

        let mut current_frame_movement = Vec3::ZERO;
        current_frame_movement.y -= 9.81 * time.delta_seconds();
        current_frame_movement += to_player_unit_vector * speed * time.delta_seconds();

        k_controller.translation = Some(current_frame_movement);
        transform.look_at(player_position, Vec3::Y);
    }
}
