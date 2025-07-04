use bevy::{app::AppExit, color::palettes::css::ALICE_BLUE, prelude::*};

use crate::{
    audio::{PlayMusicEvent, PlaybackSettings},
    AppState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (setup_main_menu, play_background_music),
        );

        app.add_systems(
            Update,
            handle_main_menu_button_interactions.run_if(in_state(AppState::MainMenu)),
        );
    }
}

#[derive(Component)]
enum MainMenuButtonAction {
    NewGame,
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/MorrisRomanAlternate-Black.ttf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        StateScoped(AppState::MainMenu),
        Children::spawn((
            Spawn(ImageNode::new(
                asset_server.load("textures/menu_background.png"),
            )),
            Spawn((
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(12.),
                    ..default()
                },
                BackgroundColor::from(ALICE_BLUE),
                MainMenuButtonAction::NewGame,
                children![(
                    Text::new("New Game"),
                    TextFont::from_font(font.clone()).with_font_size(32.0),
                    TextColor(Color::BLACK),
                )],
            )),
            #[cfg(not(target_family = "wasm"))]
            Spawn((
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(6.),
                    ..default()
                },
                BackgroundColor::from(ALICE_BLUE),
                MainMenuButtonAction::Exit,
                children![(
                    Text::new("Exit"),
                    TextFont::from_font(font).with_font_size(32.0),
                    TextColor(Color::BLACK),
                )],
            )),
        )),
    ));
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
                MainMenuButtonAction::Exit => {
                    exit.write(AppExit::Success);
                }
                MainMenuButtonAction::NewGame => {
                    app_state.set(AppState::InGame);
                }
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

fn play_background_music(mut play_music_event_writer: EventWriter<PlayMusicEvent>) {
    play_music_event_writer.write(PlayMusicEvent::new(
        "theme1.ogg",
        Some(PlaybackSettings {
            volume: 0.25,
            ..default()
        }),
        None,
    ));
}
