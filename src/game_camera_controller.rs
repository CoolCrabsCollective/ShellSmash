use bevy::math::vec3;
use bevy::prelude::*;

use crate::game::HolyCam;
use crate::game_state::GameState;
use crate::player::PlayerControllerState;

pub struct GameCameraControllerPlugin;

impl Plugin for GameCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            set_camera.run_if(in_state(GameState::FightingInArena)),
        );
    }
}

#[derive(Resource)]
struct GameCameraState {}

pub fn set_camera(
    mut camera_transform_query: Query<
        (&mut Transform, &Camera, &HolyCam),
        Without<PlayerControllerState>,
    >,
    player: Query<(&Transform, &PlayerControllerState)>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_transform_query.single_mut().0;
    let (player_transform, player_controller) = player.single();

    //camera_transform.look_at(player_transform.single().translation, Vec3::Y);
    let new_pos = player_transform.translation
        + player_transform.forward() * 1.0
        + vec3(0.0, 60.0, 30.0)
        + vec3(0.0, 0.0, -1.0);
    let mut rate = 0.0025f32.powf(time.delta_seconds());

    if new_pos.distance_squared(camera_transform.translation) > 10.0 * 10.0 {
        rate = 0.0;
    }

    camera_transform.translation = camera_transform.translation * rate + new_pos * (1.0 - rate);
}
