use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::{game_state::GameState, wave_manager::ARENA_DIMENSIONS_METERS};

pub struct ProjectilePlugin;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub direction: Vec3,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub pbr: PbrBundle,
    pub projectile: Projectile,
    pub collider: Collider,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_projectiles.run_if(in_state(GameState::FightingInArena)),
        );
    }
}

fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (entity, mut transform, Projectile { speed, direction }) in &mut projectile_query {
        transform.translation += (*speed) * (*direction) * time.delta_seconds();

        if transform.translation.length() > ARENA_DIMENSIONS_METERS[1] {
            commands.entity(entity).despawn();
        }
    }
}
