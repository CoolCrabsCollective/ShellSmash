use bevy::prelude::{Component, Res};

use crate::inventory::Inventory;

#[derive(Component)]
pub struct PlayerCombatState {
    is_attack_pressed: bool,
    auto_attack_on: bool,
    damage: i32,
    attack_speed: f32,
    current_hp: i32,
    max_hp: i32,
}

impl PlayerCombatState {
    fn compute_from_inventory(&mut self, inv: &Res<Inventory>) {
        for item in &inv.content {
            self.max_hp += item.hp_gain;
            self.attack_speed += item.attack_speed_gain;
            self.damage += item.attack_damage_gain;
        }
    }
}
