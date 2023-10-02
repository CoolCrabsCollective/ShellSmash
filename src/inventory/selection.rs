use crate::inventory::PackedInventoryItem;
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

pub fn select_next(
    query_items: Query<Entity, With<PackedInventoryItem>>,
    mut selected: ResMut<SelectedItem>,
) {
    if selected.selected_entity == None {
        selected.selected_entity = query_items.iter().next();
        return;
    }

    let id = selected.selected_entity.unwrap();
    let mut prev = None;

    for entity in query_items.iter() {
        if id < entity && (prev == None || id >= prev.unwrap()) {
            selected.selected_entity = Some(entity);
            return;
        }
        prev = Some(entity);
    }

    // set it to beginning
    selected.selected_entity = query_items.iter().next();
}
