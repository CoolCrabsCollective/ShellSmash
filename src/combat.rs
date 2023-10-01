use bevy::prelude::Component;

#[derive(Component)]
pub struct Attacker {
    damage: i32,
    attack_speed: f32,
}
