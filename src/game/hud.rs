use bevy::prelude::*;

use crate::{playing, AppState};

use super::{
    resource_pool::{Fire, Health, ResourcePool},
    Player,
};

const HEALTH_BAR_WIDTH: i16 = 150;
const HEALTH_BAR_HEIGHT: i16 = 15;

const FIRE_BREATH_BAR_WIDTH: i16 = 150;
const FIRE_BAR_HEIGHT: i16 = 15;

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_hud);

        app.add_systems(
            FixedUpdate,
            (update_health_bar_display, update_fire_bar_display).run_if(playing()),
        );
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct FireBreathBar;

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Hud,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::End,
                    padding: UiRect::all(Val::Px(16.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    border_color: BorderColor(Color::BLACK),
                    style: Style {
                        border: UiRect::all(Val::Px(2.)),
                        width: Val::Px(HEALTH_BAR_WIDTH as f32),
                        height: Val::Px(HEALTH_BAR_HEIGHT as f32),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|health_bar_builder| {
                    health_bar_builder.spawn((
                        NodeBundle {
                            background_color: BackgroundColor(Color::RED),
                            style: Style {
                                width: Val::Px(HEALTH_BAR_WIDTH as f32 - 4.),
                                height: Val::Px(HEALTH_BAR_HEIGHT as f32 - 4.),
                                ..default()
                            },
                            ..default()
                        },
                        HealthBar,
                    ));
                });

            builder
                .spawn(NodeBundle {
                    border_color: BorderColor(Color::BLACK),
                    style: Style {
                        border: UiRect::all(Val::Px(2.)),
                        width: Val::Px(FIRE_BREATH_BAR_WIDTH as f32),
                        height: Val::Px(FIRE_BAR_HEIGHT as f32),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|fire_breath_bar_builder| {
                    fire_breath_bar_builder.spawn((
                        NodeBundle {
                            background_color: BackgroundColor(Color::LIME_GREEN),
                            style: Style {
                                width: Val::Px(FIRE_BREATH_BAR_WIDTH as f32 - 4.),
                                height: Val::Px(FIRE_BAR_HEIGHT as f32 - 4.),
                                ..default()
                            },
                            ..default()
                        },
                        FireBreathBar,
                    ));
                });
        });
}

fn update_health_bar_display(
    player_query: Query<&ResourcePool<Health>, (Changed<ResourcePool<Health>>, With<Player>)>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
) {
    if let Ok(hitpoints) = player_query.get_single() {
        let mut style = health_bar_query.single_mut();

        style.width = Val::Px(HEALTH_BAR_WIDTH as f32 * hitpoints.current_percentage());
    }
}

fn update_fire_bar_display(
    player_query: Query<&ResourcePool<Fire>, (Changed<ResourcePool<Fire>>, With<Player>)>,
    mut fire_bar_query: Query<&mut Style, With<FireBreathBar>>,
) {
    if let Ok(fire_breath_resource) = player_query.get_single() {
        let mut style = fire_bar_query.single_mut();

        style.width =
            Val::Px(FIRE_BREATH_BAR_WIDTH as f32 * fire_breath_resource.current_percentage());
    }
}
