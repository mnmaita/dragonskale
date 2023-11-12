use bevy::prelude::*;

use crate::AppState;

use super::player::spawn_player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (draw_background, spawn_player));
    }
}

fn draw_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server
        .get_handle("textures/background.png")
        .unwrap_or_default();
    commands.spawn(SpriteBundle {
        texture,
        ..default()
    });
}
