use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub struct GameStatePlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    FightingInArena,
    ManagingInventory,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>();
        app.add_systems(Update, process_inputs);
    }
}

fn process_inputs(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    current_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in keyboard_input_events.iter() {
        if event.state.is_pressed() {
            match event.key_code {
                Some(KeyCode::V) => {
                    next_state.set(match current_state.get() {
                        GameState::FightingInArena => GameState::ManagingInventory,
                        GameState::ManagingInventory => GameState::FightingInArena,
                    });
                }
                _ => {}
            }
        }
    }
}
