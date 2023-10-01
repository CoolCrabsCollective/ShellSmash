use crate::game_state::GameState;
use crate::player::PlayerControllerState;
use bevy::log;
use bevy::prelude::*;
use crate::inventory::{InventoryItem};

pub struct CollectablePlugin;

type CollectedItems = InventoryItem;

#[derive(Component)]
pub struct Collectable(pub(crate) bool);

#[derive(Event)]
pub struct ItemCollectEvent(pub(crate) CollectedItems);

impl Plugin for CollectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            detect_items.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_event::<ItemCollectEvent>();
    }
}

fn detect_items(
    mut commands: Commands,
    items: Query<(Entity, &Transform, &Collectable, &InventoryItem)>,
    player_trans: Query<&Transform, With<PlayerControllerState>>,
    mut item_collected_event_writer: EventWriter<ItemCollectEvent>,
) {
    let detect_range = 0.5;
    let current_location = player_trans.single().translation;

    for item in items.iter() {
        if item.2 .0 {
            let distance_squared = current_location.distance_squared(item.1.translation);

            if distance_squared < detect_range * detect_range {
                item_collected_event_writer.send(ItemCollectEvent(item.3.clone()));
                commands.entity(item.0).despawn();
            }
        }
    }
}
