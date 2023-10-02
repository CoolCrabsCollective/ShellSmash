use bevy::prelude::Visibility::{Hidden, Visible};
use bevy::prelude::*;
use bevy::ui::PositionType::Absolute;

use crate::game_state::GameState;
use crate::inventory::selection::SelectedItem;
use crate::inventory::{selection, Inventory};

pub struct InventoryUIPlugin;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
pub struct InventoryUI;
#[derive(Component)]
pub struct ValidationButton;
#[derive(Component)]
pub struct ItemSwitch;

impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            validation_button.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(
            Update,
            select_next_button.run_if(in_state(GameState::ManagingInventory)),
        );
        app.add_systems(OnEnter(GameState::ManagingInventory), build_ui);
        app.add_systems(OnExit(GameState::ManagingInventory), clean);
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    // spawned in title screen now
    // commands.spawn(Camera2dBundle {
    //     camera: Camera { ..default() },
    //     ..default()
    // });
}

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>, inventory: Res<Inventory>) {
    let multiple_items = inventory.content.iter().count() > 1;

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: Absolute,
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(InventoryUI)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "You have limited inventory space! What does not fit in the grid, you lose.",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(InventoryUI)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        top: Val::Percent(5.0),
                        //left: Val::Percent(5.0),
                        right: Val::Percent(30.0),
                        width: Val::Percent(20.0),
                        height: Val::Percent(95.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
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
                            visibility: if multiple_items { Visible } else { Hidden },
                            ..default()
                        })
                        .insert(ItemSwitch)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Select Next",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        top: Val::Percent(5.0),
                        left: Val::Percent(30.0),
                        width: Val::Percent(20.0),
                        height: Val::Percent(95.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
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
                        .insert(ValidationButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Finish",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}

fn validation_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<ValidationButton>),
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

fn select_next_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<ItemSwitch>),
    >,
    mut text_query: Query<&mut Text>,
    mut selected: ResMut<SelectedItem>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //selection::select_next(selected);
                *color = PRESSED_BUTTON.into();
                return;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                return;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                return;
            }
        }
    }
}

fn clean(mut commands: Commands, query: Query<Entity, With<InventoryUI>>) {
    for ui_element in query.iter() {
        commands.entity(ui_element).despawn();
    }
}
