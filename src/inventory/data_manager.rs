use bevy::log;
use bevy::prelude::*;
use crate::collectable::ItemCollectEvent;
use crate::game_state::GameState;
use crate::inventory::Inventory;

pub struct InventoryDataPlugin;

impl Plugin for InventoryDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_item_collect.run_if(in_state(GameState::FightingInArena)),
        );
    }
}

pub fn handle_item_collect(
    mut item_collect_event_reader: EventReader<ItemCollectEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut inventory: ResMut<Inventory>,
) {
    if item_collect_event_reader.len() > 0 {
        for item in &mut item_collect_event_reader {
            log::info!("Player collected item: {:?}", item.0);
            inventory.content.push(item.0.clone());
        }

        let new_state = GameState::ManagingInventory;
        next_state.set(new_state);
    }
}