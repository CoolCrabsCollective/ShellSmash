use crate::config::INVENTORY_GRID_DIMENSIONS;
use crate::inventory::PackedInventoryItem;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub struct InventoryValidationPlugin;

impl Plugin for InventoryValidationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_background);
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
