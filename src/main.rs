#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use animation::AnimationPlugin;
use audio::{audio_assets_loaded, AudioPlugin, BgmChannel};
use bevy::{
    ecs::query::QueryFilter,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::{AudioChannel, AudioControl};
use camera::CameraPlugin;
use fonts::{font_assets_loaded, FontsPlugin};
use game::GamePlugin;
use input::InputPlugin;
use main_menu::MainMenuPlugin;
use physics::PhysicsPlugin;
use textures::{texture_assets_loaded, TexturesPlugin};

mod animation;
mod audio;
mod camera;
#[cfg(debug_assertions)]
mod debug;
mod fonts;
mod game;
mod input;
mod main_menu;
mod physics;
mod textures;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        },
        DefaultPlugins
            // FIXME: Remove setting the backend explicitly to avoid noisy warnings
            // when https://github.com/gfx-rs/wgpu/issues/3959 gets fixed.
            .set(
                #[cfg(not(target_family = "wasm"))]
                RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::DX12),
                        ..default()
                    }),
                    ..default()
                },
                #[cfg(target_family = "wasm")]
                RenderPlugin::default(),
            )
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                mode: AssetMode::Unprocessed,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "DragonSkale".into(),
                    ..default()
                }),
                ..default()
            }),
        AnimationPlugin,
        AudioPlugin,
        CameraPlugin,
        #[cfg(debug_assertions)]
        debug::DebugPlugin,
        FontsPlugin,
        GamePlugin,
        InputPlugin,
        MainMenuPlugin,
        PhysicsPlugin,
        TexturesPlugin,
    ));

    // app.world().remove_resource::<EventUpdateSignal>();

    app.init_state::<AppState>();

    app.insert_resource(Msaa::Off);

    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.add_systems(
        Update,
        handle_asset_load.run_if(assets_loaded().and_then(run_once())),
    );

    app.add_systems(
        Update,
        stop_music_on_transition.run_if(state_changed::<AppState>),
    );

    app.run();
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum AppState {
    #[default]
    Setup,
    MainMenu,
    InGame,
    GameOver,
}

fn handle_asset_load(mut state: ResMut<NextState<AppState>>) {
    #[cfg(debug_assertions)]
    info!("Assets loaded successfully.");
    state.set(AppState::MainMenu);
}

pub fn entity_cleanup<F: QueryFilter>(mut commands: Commands, query: Query<Entity, F>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn stop_music_on_transition(bgm_audio_channel: Res<AudioChannel<BgmChannel>>) {
    bgm_audio_channel.stop();
}

pub fn playing() -> impl Condition<()> {
    IntoSystem::into_system(in_state(AppState::InGame))
}

fn assets_loaded() -> impl Condition<()> {
    texture_assets_loaded()
        .and_then(audio_assets_loaded())
        .and_then(font_assets_loaded())
}
