use crate::config::INVENTORY_GRID_DIMENSIONS;
use crate::game_state::GameState;
use crate::inventory::{Inventory, InventoryItem, PackedInventoryItem};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub struct InventoryValidationPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
pub struct ValidationButton;

impl Plugin for InventoryValidationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_background.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            button_system.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(OnExit(GameState::ManagingInventory), save_and_clear_render);
        app.add_systems(OnEnter(GameState::ManagingInventory), build_ui);
        app.add_systems(OnExit(GameState::ManagingInventory), clean);
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera { ..default() },
        ..default()
    });
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

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Percent(5.0),
                left: Val::Percent(80.0),
                width: Val::Percent(20.0),
                height: Val::Percent(95.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(ValidationButton)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Continue",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::FightingInArena);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn clean(mut commands: Commands, query: Query<Entity, With<ValidationButton>>) {
    for ui_element in query.iter() {
        commands.entity(ui_element).despawn();
    }
}
