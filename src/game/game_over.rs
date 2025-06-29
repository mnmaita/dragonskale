use bevy::prelude::*;

use crate::{
    audio::{PlayMusicEvent, PlaybackSettings},
    playing, AppState,
};

use super::{
    resource_pool::{Health, ResourcePool},
    score_system::Score,
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
                (fade_out_screen, fade_in_text).run_if(in_state(AppState::GameOver)),
            ),
        );

        app.add_systems(
            OnEnter(AppState::GameOver),
            (
                play_background_music,
                display_game_over_screen,
                update_score_display.after(display_game_over_screen),
            ),
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

#[derive(Component)]
struct ScoreDisplay;

fn check_game_over_condition(
    mut next_state: ResMut<NextState<AppState>>,
    player_health: Single<&ResourcePool<Health>, (With<Player>, Changed<ResourcePool<Health>>)>,
) {
    if player_health.current() == 0 {
        next_state.set(AppState::GameOver);
    }
}

fn display_game_over_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            GameOverBackground,
            StateScoped(AppState::GameOver),
            Node {
                align_items: AlignItems::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.)),
        ))
        .with_children(|builder| {
            builder.spawn((
                GameOverText,
                Text::new("Game Over"),
                TextColor(Color::WHITE.with_alpha(0.0)),
                TextFont::from_font(
                    asset_server
                        .get_handle("fonts/MorrisRomanAlternate-Black.ttf")
                        .unwrap_or_default(),
                )
                .with_font_size(64.0),
            ));

            builder.spawn((
                GameOverText,
                ScoreDisplay,
                Text::new("Score:"),
                TextColor(Color::WHITE.with_alpha(0.0)),
                TextFont::from_font(
                    asset_server
                        .get_handle("fonts/MorrisRomanAlternate-Black.ttf")
                        .unwrap_or_default(),
                )
                .with_font_size(32.0),
            ));

            builder
                .spawn((
                    Button,
                    BackgroundColor(Color::default().with_alpha(0.)),
                    GameOverButtonAction::BackToMenu,
                ))
                .with_children(|button| {
                    button.spawn((
                        GameOverText,
                        Text::new("Back to Menu"),
                        TextColor(Color::WHITE.with_alpha(0.0)),
                        TextFont::from_font(
                            asset_server
                                .get_handle("fonts/MorrisRomanAlternate-Black.ttf")
                                .unwrap_or_default(),
                        )
                        .with_font_size(32.0),
                    ));
                });
        });
}

fn fade_out_screen(mut background_color: Single<&mut BackgroundColor, With<GameOverBackground>>) {
    if background_color.0.alpha() < 1. {
        let alpha = background_color.0.alpha();
        background_color.0.set_alpha(alpha + 0.01);
    }
}

fn update_score_display(
    mut score_text: Single<&mut Text, (With<GameOverText>, With<ScoreDisplay>)>,
    player_score: Single<&Score, With<Player>>,
) {
    score_text.0 = format!("Score: {}", player_score.current());
}

fn fade_in_text(
    mut query: Query<&mut TextColor, With<GameOverText>>,
    background_color: Single<&BackgroundColor, With<GameOverBackground>>,
) {
    for mut text_color in &mut query {
        if background_color.0.alpha() >= 0.5 && text_color.alpha() < 1. {
            let alpha = text_color.alpha();
            text_color.set_alpha(alpha + 0.001);
        }
    }
}

fn play_background_music(mut play_music_event_writer: EventWriter<PlayMusicEvent>) {
    play_music_event_writer.write(PlayMusicEvent::new(
        "theme3.ogg",
        Some(PlaybackSettings {
            volume: 0.25,
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
