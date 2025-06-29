use bevy::{
    color::palettes::css::{GOLD, LIMEGREEN, RED},
    prelude::*,
};

use crate::{playing, AppState};

use super::{
    resource_pool::{Fire, Health, ResourcePool},
    score_system::Score,
    Player,
};

const BAR_WIDTH: f32 = 150.;
const BAR_HEIGHT: f32 = 15.;
const BAR_BORDER_SIZE: f32 = 2.;

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_hud);

        app.add_systems(
            FixedUpdate,
            (
                update_health_bar_display,
                update_fire_bar_display,
                update_score_display,
            )
                .run_if(playing()),
        );
    }
}

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct FireBreathBar;

#[derive(Component)]
struct ScoreDisplay;

fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            StateScoped(AppState::GameOver),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::End,
                padding: UiRect::all(Val::Px(16.)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Node {
                        border: UiRect::all(Val::Px(BAR_BORDER_SIZE)),
                        width: Val::Px(BAR_WIDTH),
                        height: Val::Px(BAR_HEIGHT),
                        ..default()
                    },
                    BorderColor::from(Color::BLACK),
                ))
                .with_children(|health_bar_builder| {
                    health_bar_builder.spawn((
                        Node {
                            width: Val::Px(BAR_WIDTH - BAR_BORDER_SIZE * 2.),
                            height: Val::Px(BAR_HEIGHT - BAR_BORDER_SIZE * 2.),
                            ..default()
                        },
                        BackgroundColor::from(RED),
                        HealthBar,
                    ));
                });

            builder
                .spawn((
                    Node {
                        border: UiRect::all(Val::Px(BAR_BORDER_SIZE)),
                        width: Val::Px(BAR_WIDTH),
                        height: Val::Px(BAR_HEIGHT),
                        ..default()
                    },
                    BorderColor::from(Color::BLACK),
                ))
                .with_children(|fire_breath_bar_builder| {
                    fire_breath_bar_builder.spawn((
                        Node {
                            width: Val::Px(BAR_WIDTH - BAR_BORDER_SIZE * 2.),
                            height: Val::Px(BAR_HEIGHT - BAR_BORDER_SIZE * 2.),
                            ..default()
                        },
                        BackgroundColor::from(LIMEGREEN),
                        FireBreathBar,
                    ));
                });
        });

    // Score text in bottom middle of screen
    commands.spawn((
        StateScoped(AppState::GameOver),
        ScoreDisplay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(5.),
            height: Val::Px(5.),
            ..default()
        },
        Text::new("Score: 0"),
        TextFont::from_font(
            asset_server
                .get_handle("fonts/Prince Valiant.ttf")
                .unwrap_or_default(),
        )
        .with_font_size(40.0),
        TextColor(GOLD.into()),
        TextLayout::new_with_no_wrap(),
    ));
}

fn update_health_bar_display(
    player_query: Query<&ResourcePool<Health>, (Changed<ResourcePool<Health>>, With<Player>)>,
    mut health_bar_query: Query<&mut Node, With<HealthBar>>,
) {
    if let Ok(hitpoints) = player_query.get_single() {
        let mut node = health_bar_query.single_mut();

        node.width = Val::Px(BAR_WIDTH * hitpoints.current_percentage());
    }
}

fn update_fire_bar_display(
    player_query: Query<&ResourcePool<Fire>, (Changed<ResourcePool<Fire>>, With<Player>)>,
    mut fire_bar_query: Query<&mut Node, With<FireBreathBar>>,
) {
    if let Ok(fire_breath_resource) = player_query.get_single() {
        let mut node = fire_bar_query.single_mut();

        node.width = Val::Px(BAR_WIDTH * fire_breath_resource.current_percentage());
    }
}

fn update_score_display(
    player_query: Query<&Score, (Changed<Score>, With<Player>)>,
    mut score_text_display_query: Query<&mut Text, With<ScoreDisplay>>,
) {
    if let Ok(score_system) = player_query.get_single() {
        let mut score_text = score_text_display_query.single_mut();
        score_text.0 = format!(
            "Score: {} - Multiplier x {}",
            score_system.current(),
            score_system.multiplier()
        );
    }
}
