use crate::game_state::GameState;
use crate::inventory::InventoryItem;
use crate::player::PlayerControllerState;
use crate::world_item::Collectable;
use bevy::log;
use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;

pub struct CollectablePlugin;

type CollectedItems = Vec<InventoryItem>;

#[derive(Event)]
pub struct ItemCollectEvent(CollectedItems);

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
    items: Query<&InventoryItem, With<Collectable>>,
    mut controllers: Query<&mut KinematicCharacterController, With<PlayerControllerState>>,
    mut item_collected_event_writer: EventWriter<ItemCollectEvent>,
) {
    let detect_range = 5.0;

    let mut near_items: Vec<InventoryItem> = vec![];
    let current_location = controllers.single_mut().translation;

    if current_location.is_some() {
        let unwrap_location = current_location.unwrap();

        for item in items.iter() {
            if item.real_location.is_some() {
                let distance_squared =
                    unwrap_location.distance_squared(item.real_location.unwrap());

                if distance_squared < detect_range * detect_range {
                    near_items.push(item.clone());
                    // remove item from world
                }
            }
        }
    }

    item_collected_event_writer.send(ItemCollectEvent(near_items));
}

fn handle_item_collect(mut item_collect_event_reader: EventReader<ItemCollectEvent>) {
    for item_collect_event in &mut item_collect_event_reader {
        log::info!("Item collected by player: {:?}", item_collect_event.0);
    }
}
