use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

use crate::config::INVENTORY_GRID_DIMENSIONS;
use crate::game_state::GameState;
use crate::inventory::{Inventory, InventoryItem, PackedInventoryItem};

pub struct InventoryValidationPlugin;

impl Plugin for InventoryValidationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_background.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(OnExit(GameState::ManagingInventory), save_and_clear_render);
    }
}

fn update_background(mut color: ResMut<ClearColor>, query: Query<&PackedInventoryItem>) {
    let mut set: HashSet<IVec3> = HashSet::new();

    let mut overlap = false;

    'bigloop: for element in query.iter() {
        for point in &element.data.local_points {
            let vec = *point + element.data.location;

            if vec.x < 0
                || vec.x >= INVENTORY_GRID_DIMENSIONS[0]
                || vec.y < 0
                || vec.y >= INVENTORY_GRID_DIMENSIONS[1]
                || vec.z < 0
                || vec.z >= INVENTORY_GRID_DIMENSIONS[2]
            {
                overlap = true;
                break;
            }

            if set.contains(&vec) {
                overlap = true;
                break 'bigloop;
            }

            set.insert(vec);
        }
    }

    color.0 = if overlap {
        Color::rgb(0.9, 0.6, 0.3)
    } else {
        Color::rgb(0.3, 0.6, 0.9)
    };
}

fn save_and_clear_render(
    mut commands: Commands,
    rendered_inventory: Query<(Entity, &PackedInventoryItem)>,
    mut inventory: ResMut<Inventory>,
) {
    inventory.content.clear();

    let mut map: HashMap<IVec3, i32> = HashMap::new();
    let mut non_overlapping = HashSet::new();

    let all: Vec<InventoryItem> = rendered_inventory
        .iter()
        .map(|x| x.1.data.clone())
        .collect();
    let mut i = 0;
    for elem in &all {
        non_overlapping.insert(i);
        i += 1;
    }

    i = 0;
    for element in &all {
        for point in &element.local_points {
            let vec = *point + element.location;

            if vec.x < 0
                || vec.x >= INVENTORY_GRID_DIMENSIONS[0]
                || vec.y < 0
                || vec.y >= INVENTORY_GRID_DIMENSIONS[1]
                || vec.z < 0
                || vec.z >= INVENTORY_GRID_DIMENSIONS[2]
            {
                non_overlapping.remove(&i);
                continue;
            }

            if map.contains_key(&vec) {
                non_overlapping.remove(&i);
                non_overlapping.remove(map.get(&vec).unwrap());
                continue;
            }

            map.insert(vec, i);
        }
        i += 1;
    }

    for index in non_overlapping {
        inventory
            .content
            .push(all.get(index as usize).unwrap().clone());
    }

    for item in rendered_inventory.iter() {
        commands.entity(item.0).despawn();
    }
}
