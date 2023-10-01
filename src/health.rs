use bevy::prelude::Component;

#[derive(Component)]
pub struct Health {
    current_hp: i32,
    max_hp: i32,
}
