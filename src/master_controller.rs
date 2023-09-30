use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub struct MasterControllerPlugin;

impl Plugin for MasterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_inputs);
        app.insert_resource(ControlState::new());
    }
}

enum Controller {
    INVENTORY,
}

#[derive(Resource)]
struct ControlState {
    controller: Controller,
}

impl ControlState {
    pub fn new() -> Self {
        Self {
            controller: Controller::INVENTORY,
        }
    }
}

fn process_inputs(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut state: ResMut<ControlState>,
) {
    for event in keyboard_events.iter() {
        match event.key_code {
            Some(KeyCode::Q) => {
                println!("test: Q");
            }
            _ => {}
        }
    }
}
