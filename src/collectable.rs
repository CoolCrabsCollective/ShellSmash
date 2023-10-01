use crate::game_state::GameState;
use crate::player::PlayerControllerState;
use crate::world_item::Collectable;
use bevy::log;
use bevy::prelude::*;

pub struct CollectablePlugin;

type CollectedItems = Entity;

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
        let distance_squared = current_location.distance_squared(item.1.translation);

        if distance_squared < detect_range * detect_range {
            item_collected_event_writer.send(ItemCollectEvent(item.0));
        }
    }
}

pub fn handle_item_collect(
    mut commands: Commands,
    mut item_collect_event_reader: EventReader<ItemCollectEvent>,
    current_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if item_collect_event_reader.len() > 0 {
        for item in &mut item_collect_event_reader {
            commands.entity(item.0).despawn();
        }

        let new_state = match current_state.get() {
            GameState::FightingInArena => GameState::ManagingInventory,
            GameState::ManagingInventory => GameState::FightingInArena,
            GameState::Loading => GameState::Loading,
        };
        log::info!("Changing game state to: {new_state:?}");
        next_state.set(new_state);
    }
}
