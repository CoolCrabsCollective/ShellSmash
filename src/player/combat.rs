use crate::game_state::GameState;
use bevy::app::{App, Plugin};
use bevy::prelude::{
    dbg, in_state, Component, Input, IntoSystemConfigs, KeyCode, MouseButton, Query, Res, Update,
};
use bevy::time::Time;
use bevy_rapier3d::na::clamp;

use crate::inventory::Inventory;

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
    pub auto_attack_on: bool,
    pub damage: i32,
    pub attack_speed: f32,
    pub current_hp: i32,
    pub max_hp: i32,
    pub last_attack: f32,
}

impl PlayerCombatState {
    pub fn new() -> Self {
        Self {
            auto_attack_on: false,
            damage: 1,
            attack_speed: 1.0,
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
        let anim_duration = BASE_ATTACK_COOLDOWN / self.attack_speed;
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
    mut player: Query<&mut PlayerCombatState>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    let mut player = player.single_mut();

    if player.last_attack + BASE_ATTACK_COOLDOWN / player.attack_speed > time.elapsed_seconds() {
        return; // too recent to attack again
    }

    if buttons.just_pressed(MouseButton::Left)
        || (buttons.pressed(MouseButton::Left) && player.auto_attack_on)
    {
        player.last_attack = time.elapsed_seconds();
    }
}
