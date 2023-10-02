use bevy::prelude::*;

use crate::{
    asset_loader::{AssetLoaderPlugin, GameAssets},
    game::HolyCam,
    game_state::GameState,
    inventory::ui::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
};

pub struct TitleScreenPlugin;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct StartGameButtonText;

#[derive(Component)]
pub struct TitleScreenUi;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetLoaderPlugin);

        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(GameState::TitleScreen), on_enter);
        app.add_systems(OnExit(GameState::TitleScreen), clean);
        app.add_systems(
            Update,
            start_game_click_handler.run_if(in_state(GameState::TitleScreen)),
        );
        app.add_systems(
            Update,
            update_start_game_button_text.run_if(in_state(GameState::TitleScreen)),
        );
        app.add_systems(
            Update,
            move_holy_camera.run_if(in_state(GameState::TitleScreen)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera { ..default() },
        ..default()
    });
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(TitleScreenUi)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("radial_background.png"),
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    height: Val::Px(250.0),
                    width: Val::Px(500.0),
                    ..default()
                },
                ..default()
            });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    // border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..default()
                })
                .insert(StartGameButton)
                .insert(TitleScreenUi)
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "Start Game",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(StartGameButtonText)
                        .insert(TitleScreenUi);
                });

            parent.spawn((TextBundle::from_section(
                "Made in Rust!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 48.0,
                    color: Color::ORANGE_RED,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            }),));
        });
}

fn start_game_click_handler(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<StartGameButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
    game_assets: Res<GameAssets>,
) {
    if !game_assets.are_all_assets_loaded() {
        return;
    }
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let color = match *interaction {
            Interaction::Pressed => Color::rgb(0.5, 0.5, 0.5),
            Interaction::Hovered => Color::rgb(0.8, 0.8, 0.8),
            Interaction::None => Color::rgb(0.9, 0.9, 0.9),
        };

        for child in children {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.sections[0].style.color = color;
            }
        }

        if *interaction == Interaction::Pressed {
            next_state.set(GameState::FightingInArena);
        }
    }
}

fn update_start_game_button_text(
    mut text_query: Query<&mut Text, With<StartGameButtonText>>,
    game_assets: Res<GameAssets>,
) {
    text_query.single_mut().sections[0].value = (if game_assets.are_all_assets_loaded() {
        "Start Game"
    } else {
        "Loading..."
    })
    .to_string();
}

fn clean(mut commands: Commands, query: Query<Entity, With<TitleScreenUi>>) {
    for ui_element in query.iter() {
        commands.entity(ui_element).despawn();
    }
}

fn move_holy_camera(mut camera_transform: Query<&mut Transform, With<HolyCam>>) {
    // camera_transform.single_mut().translation = Vec3::new(-500.0, -500.0, 0.0);
}
