use bevy::{app::AppExit, prelude::*};

use crate::{AppState, InState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu);
        app.add_systems(
            Update,
            handle_main_menu_button_interactions.run_if(in_state(AppState::MainMenu)),
        );
    }
}

#[derive(Component)]
enum MainMenuButtonAction {
    NewGame,
    Settings,
    // TODO: Do not compile in WASM builds
    Exit,
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            InState(AppState::MainMenu),
        ))
        .with_children(|node| {
            node.spawn((
                ButtonBundle {
                    background_color: Color::ALICE_BLUE.into(),
                    ..default()
                },
                MainMenuButtonAction::NewGame,
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "New Game",
                        TextStyle {
                            color: Color::BLACK,
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });

            node.spawn((
                ButtonBundle {
                    background_color: Color::ALICE_BLUE.into(),
                    ..default()
                },
                MainMenuButtonAction::Settings,
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "Settings",
                        TextStyle {
                            color: Color::BLACK,
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });

            // TODO: Do not compile in WASM builds
            node.spawn((
                ButtonBundle {
                    background_color: Color::ALICE_BLUE.into(),
                    ..default()
                },
                MainMenuButtonAction::Exit,
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "Exit",
                        TextStyle {
                            color: Color::BLACK,
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });
        });
}

fn handle_main_menu_button_interactions(
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    query: Query<(&Interaction, &MainMenuButtonAction), With<Button>>,
) {
    for (interaction, main_menu_button_action) in query.iter() {
        match interaction {
            Interaction::Pressed => match main_menu_button_action {
                // TODO: Do not compile in WASM builds
                MainMenuButtonAction::Exit => exit.send(AppExit),
                MainMenuButtonAction::NewGame => app_state.set(AppState::InGame),
                MainMenuButtonAction::Settings => (),
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
