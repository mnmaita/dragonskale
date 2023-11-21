use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};

use crate::{audio::PlayMusicEvent, playing, AppState, InState};

use super::{
    resource_pool::{Health, ResourcePool},
    Player,
};

pub(super) struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_game_over_button_interactions);

        app.add_systems(
            FixedUpdate,
            (
                check_game_over_condition.run_if(playing()),
                (fade_out_screen, fade_in_text)
                    .chain()
                    .run_if(in_state(AppState::GameOver)),
            ),
        );

        app.add_systems(
            OnEnter(AppState::GameOver),
            (play_background_music, display_game_over_screen),
        );
    }
}

#[derive(Component)]
enum GameOverButtonAction {
    BackToMenu,
}

#[derive(Component)]
struct GameOverBackground;

#[derive(Component)]
struct GameOverText;

fn check_game_over_condition(
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<&ResourcePool<Health>, (With<Player>, Changed<ResourcePool<Health>>)>,
) {
    if let Ok(player_health) = query.get_single() {
        if player_health.current() == 0 {
            next_state.set(AppState::GameOver);
        }
    }
}

fn display_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            GameOverBackground,
            InState(AppState::GameOver),
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.)),
                style: Style {
                    align_items: AlignItems::Center,
                    display: Display::Flex,
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                GameOverText,
                TextBundle {
                    text: Text::from_section(
                        "Game Over",
                        TextStyle {
                            color: Color::WHITE.with_a(0.),
                            font_size: 64.,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
        });
}

fn fade_out_screen(mut query: Query<&mut BackgroundColor, With<GameOverBackground>>) {
    let mut background_color = query.single_mut();

    if background_color.0.a() < 1. {
        let alpha = background_color.0.a();
        background_color.0.set_a(alpha + 0.01);
    }
}

fn fade_in_text(
    mut query: Query<&mut Text, With<GameOverText>>,
    background_query: Query<&BackgroundColor, With<GameOverBackground>>,
) {
    let mut text = query.single_mut();
    let background_color = background_query.single();

    if background_color.0.a() >= 0.5 && text.sections[0].style.color.a() < 1. {
        let alpha = text.sections[0].style.color.a();
        text.sections[0].style.color.set_a(alpha + 0.001);
    }
}

fn play_background_music(mut play_music_event_writer: EventWriter<PlayMusicEvent>) {
    play_music_event_writer.send(PlayMusicEvent::new(
        "theme3.ogg",
        Some(PlaybackSettings {
            volume: Volume::Absolute(VolumeLevel::new(0.25)),
            ..default()
        }),
        None,
    ));
}

fn handle_game_over_button_interactions(
    mut app_state: ResMut<NextState<AppState>>,
    query: Query<(&Interaction, &GameOverButtonAction), With<Button>>,
) {
    for (interaction, game_over_button_action) in query.iter() {
        match interaction {
            Interaction::Pressed => match game_over_button_action {
                GameOverButtonAction::BackToMenu => app_state.set(AppState::MainMenu),
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
