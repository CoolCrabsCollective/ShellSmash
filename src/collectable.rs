use crate::game_state::GameState;
use crate::player::PlayerControllerState;
use crate::world_item::Collectable;
use bevy::log;
use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;
use crate::inventory::InventoryItem;

pub struct CollectablePlugin;

type CollectedItems = Vec<Entity>;

#[derive(Event)]
pub struct ItemCollectEvent(CollectedItems);

impl Plugin for CollectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            detect_items.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_systems(
            Update,
            handle_item_collect.run_if(in_state(GameState::FightingInArena)),
        );
        app.add_event::<ItemCollectEvent>();
    }
}

fn detect_items(
    items: Query<(Entity, &Transform, &Collectable)>,
    player_trans: Query<&Transform, With<PlayerControllerState>>,
    mut item_collected_event_writer: EventWriter<ItemCollectEvent>,
) {
    let detect_range = 0.5;

    let mut near_items: Vec<Entity> = vec![];
    let current_location = player_trans.single().translation;

    for item in items.iter() {
        let distance_squared =
            current_location.distance_squared(item.1.translation);

        if distance_squared < detect_range * detect_range {
            near_items.push(item.0);
        }
    }

    if near_items.len() > 0 {
        item_collected_event_writer.send(ItemCollectEvent(near_items));
    }
}

fn handle_item_collect(mut item_collect_event_reader: EventReader<ItemCollectEvent>) {
    for item_collect_event in &mut item_collect_event_reader {
        log::info!("Item collected by player: {:?}", item_collect_event.0);
    }
}
