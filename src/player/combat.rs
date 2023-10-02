use crate::enemy::Enemy;
use crate::game_state::GameState;
use bevy::audio::PlaybackMode::{Despawn, Once};
use bevy::audio::Volume::Relative;
use bevy::audio::VolumeLevel;
use bevy::prelude::*;
use bevy::time::Time;
use bevy_rapier3d::na::clamp;

use crate::inventory::Inventory;
use crate::inventory::ItemType::MELEE_WEAPON;
use crate::inventory::ItemTypeId::WillSword;
use crate::player::PlayerState;
use crate::world_item::WeaponHolder;

pub const BASE_ATTACK_COOLDOWN: f32 = 0.5;

// how long it takes to regen 1 heart
pub const PLAYER_HEAL_COOLDOWN: f32 = 5.0;

pub const PLAYER_INVICIBILITY_COOLDOWN: f32 = 2.0;

pub struct PlayerCombatPlugin;

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            process_hit.run_if(
                in_state(GameState::FightingInArena).and_then(in_state(PlayerState::Fighting)),
            ),
        );
        app.add_systems(
            Update,
            player_heal.run_if(in_state(GameState::FightingInArena)),
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
    pub last_heal: f32,
    pub last_hit: f32,
}

fn player_heal(mut player: Query<&mut PlayerCombatState>, time: Res<Time>) {
    let mut player = player.single_mut();

    if player.current_hp == player.max_hp {
        return;
    }

    if player.last_heal + PLAYER_HEAL_COOLDOWN > time.elapsed_seconds() {
        return;
    }

    player.current_hp += 1;
    player.last_heal = time.elapsed_seconds();
}

impl PlayerCombatState {
    pub fn new() -> Self {
        Self {
            damage: 1,
            attack_speed: 1.0,
            current_weapon_attack_speed: 1.0,
            current_hp: 3,
            max_hp: 3,
            last_attack: -10000.0,
            last_heal: -10000.0,
            last_hit: -10000.0,
        }
    }

    pub fn compute_from_inventory(&mut self, inv: &Res<Inventory>) {
        self.max_hp = 3;
        self.attack_speed = 1.0;
        self.damage = 1;

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
    mut asset_server: ResMut<AssetServer>,
    mut player: Query<(&Transform, &mut PlayerCombatState, &WeaponHolder)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    buttons: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    gamepad_buttons: Res<Input<GamepadButton>>,
) {
    let mut player = player.single_mut();

    if player.2.current_weapon.is_none() {
        return;
    }

    let current_weapon = player.2.current_weapon.clone().unwrap().1;

    if current_weapon.item_type != MELEE_WEAPON {
        return;
    }

    if player.1.last_attack
        + BASE_ATTACK_COOLDOWN / (player.1.attack_speed * current_weapon.weapon_attack_speed)
        > time.elapsed_seconds()
    {
        return; // too recent to attack again
    }

    let distance_to_kill = if current_weapon.item_type_id == WillSword {
        1.3
    } else {
        1.9
    };

    let mut gamepad_hit = false;

    for gamepad in gamepads.iter() {
        if gamepad_buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        }) || gamepad_buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::RightTrigger,
        }) || gamepad_buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::RightTrigger2,
        }) || gamepad_buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::RightThumb,
        }) {
            gamepad_hit = true;
            break;
        }
    }

    if buttons.just_pressed(MouseButton::Left)
        || (buttons.pressed(MouseButton::Left) && current_weapon.weapon_is_auto)
        || touches.first_pressed_position() != None
        || gamepad_hit
    {
        commands.spawn(AudioBundle {
            source: asset_server.load("swing.ogg"),
            settings: PlaybackSettings {
                mode: Despawn,
                volume: Relative(VolumeLevel::new(0.5f32)),
                ..default()
            },
            ..default()
        });
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
