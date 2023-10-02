use crate::config::INVENTORY_GRID_DIMENSIONS;
use crate::game_state::GameState;
use crate::inventory::PackedInventoryItem;
use bevy::prelude::*;
use bevy::utils::HashSet;

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
        app.add_systems(OnEnter(GameState::ManagingInventory), build_ui);
        app.add_systems(OnExit(GameState::ManagingInventory), clean);
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

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera: Camera { ..default() },
        ..default()
    });
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
