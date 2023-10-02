use crate::enemy::Enemy;
use crate::game_state::GameState;
use bevy::prelude::*;
use bevy::time::Time;
use bevy_rapier3d::na::clamp;

use crate::inventory::Inventory;
use crate::world_item::WeaponHolder;

pub const BASE_ATTACK_COOLDOWN: f32 = 0.5;

pub struct PlayerCombatPlugin;

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            process_hit.run_if(
                in_state(GameState::FightingInArena), //.and_then(in_state(PlayerState::Fighting)),
            ),
        );
    }
}

#[derive(Component, Clone)]
pub struct PlayerCombatState {
    pub damage: i32,
    pub attack_speed: f32,
    pub current_weapon_attack_speed: f32,
    pub current_hp: i32,
    pub max_hp: i32,
    pub last_attack: f32,
}

impl PlayerCombatState {
    pub fn new() -> Self {
        Self {
            damage: 1,
            attack_speed: 1.0,
            current_weapon_attack_speed: 1.0,
            current_hp: 5,
            max_hp: 5,
            last_attack: -10000.0,
        }
    }

    pub fn compute_from_inventory(&mut self, inv: &Res<Inventory>) {
        for item in &inv.content {
            self.max_hp += item.hp_gain;
            self.attack_speed *= item.attack_speed_gain;
            self.damage += item.attack_damage_gain;
        }
    }

    pub fn get_weapon_angle(&self, time: &Res<Time>) -> f32 {
        let anim_duration =
            BASE_ATTACK_COOLDOWN / (self.attack_speed * self.current_weapon_attack_speed);
        let anim_progress = 1.0
            - clamp(
                ((self.last_attack + anim_duration) - time.elapsed_seconds()) / anim_duration,
                0.0,
                1.0,
            );

        return if anim_progress < 0.5 {
            let anim_progress = anim_progress / 0.5;
            (anim_progress * 140.0 - 70.0).to_radians()
        } else {
            let anim_progress = (anim_progress - 0.5) / 0.5;
            ((1.0 - anim_progress) * 140.0 - 70.0).to_radians()
        };
    }
}

fn process_hit(
    mut commands: Commands,
    mut player: Query<(&Transform, &mut PlayerCombatState, &WeaponHolder)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let mut player = player.single_mut();

    if player.2.current_weapon.is_none() {
        return;
    }

    let current_weapon = player.2.current_weapon.clone().unwrap().1;

    if player.1.last_attack
        + BASE_ATTACK_COOLDOWN / (player.1.attack_speed * current_weapon.weapon_attack_speed)
        > time.elapsed_seconds()
    {
        return; // too recent to attack again
    }

    let distance_to_kill = 1.9;

    if buttons.just_pressed(MouseButton::Left)
        || (buttons.pressed(MouseButton::Left) && current_weapon.weapon_is_auto)
    {
        for enemy in enemies.iter() {
            if enemy
                .1
                .translation
                .distance_squared(player.0.translation + player.0.forward() * 1.0)
                < distance_to_kill * distance_to_kill
            {
                commands.entity(enemy.0).despawn();
            }
        }

        player.1.last_attack = time.elapsed_seconds();
        player.1.current_weapon_attack_speed = current_weapon.weapon_attack_speed;
    }
}
