use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedItem {
    pub selected_entity: Option<Entity>,
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedItem {
            selected_entity: None,
        });
    }
}
