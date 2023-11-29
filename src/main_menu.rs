use bevy::{
    app::AppExit,
    audio::{Volume, VolumeLevel},
    prelude::*,
};

use crate::{audio::PlayMusicEvent, entity_cleanup, AppState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (
                setup_main_menu,
                play_background_music.after(setup_main_menu),
            ),
        );

        app.add_systems(
            Update,
            handle_main_menu_button_interactions.run_if(in_state(AppState::MainMenu)),
        );

        app.add_systems(
            OnExit(AppState::MainMenu),
            entity_cleanup::<With<MainMenuEntity>>,
        );
    }
}

#[derive(Component)]
struct MainMenuEntity;

#[derive(Component)]
enum MainMenuButtonAction {
    NewGame,
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            MainMenuEntity,
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
                            font: asset_server
                                .get_handle("fonts/MorrisRomanAlternate-Black.ttf")
                                .unwrap_or_default(),
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });

            #[cfg(not(target_family = "wasm"))]
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
                            font: asset_server
                                .get_handle("fonts/MorrisRomanAlternate-Black.ttf")
                                .unwrap_or_default(),
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
                #[cfg(not(target_family = "wasm"))]
                MainMenuButtonAction::Exit => exit.send(AppExit),
                MainMenuButtonAction::NewGame => app_state.set(AppState::InGame),
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

fn play_background_music(mut play_music_event_writer: EventWriter<PlayMusicEvent>) {
    play_music_event_writer.send(PlayMusicEvent::new(
        "theme1.ogg",
        Some(PlaybackSettings {
            volume: Volume::Absolute(VolumeLevel::new(0.25)),
            ..default()
        }),
        None,
    ));
}
