use animation::AnimationPlugin;
use audio::{audio_assets_loaded, AudioPlugin};
use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use camera::CameraPlugin;
use game::GamePlugin;
use input::InputPlugin;
use physics::PhysicsPlugin;
use textures::{texture_assets_loaded, TexturesPlugin};

mod animation;
mod audio;
mod camera;
mod game;
mod input;
mod physics;
mod textures;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        // FIXME: Remove setting the backend explicitly to avoid noisy warnings
        // when https://github.com/gfx-rs/wgpu/issues/3959 gets fixed.
        DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
        }),
        AnimationPlugin,
        AudioPlugin,
        CameraPlugin,
        GamePlugin,
        InputPlugin,
        PhysicsPlugin,
        TexturesPlugin,
    ));

    app.add_state::<AppState>();

    app.add_systems(
        Update,
        handle_asset_load.run_if(assets_loaded().and_then(run_once())),
    );

    app.run();
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum AppState {
    #[default]
    Setup,
    InGame,
}

pub fn playing() -> impl Condition<()> {
    IntoSystem::into_system(in_state(AppState::InGame))
}

fn handle_asset_load(mut state: ResMut<NextState<AppState>>) {
    #[cfg(debug_assertions)]
    info!("Assets loaded successfully.");
    state.set(AppState::InGame);
}

fn assets_loaded() -> impl Condition<()> {
    texture_assets_loaded().and_then(audio_assets_loaded())
}
