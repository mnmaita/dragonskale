use bevy::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    AppState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server
        .get_handle("textures/dragon.png")
        .unwrap_or_default();
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(191., 161.), 12, 1, None, None);
    let texture_atlas_handle = asset_server.add(texture_atlas);

    commands.spawn((
        Player,
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec2::ZERO.extend(1.)),
            ..default()
        },
        AnimationIndices::new(0, 2),
        AnimationTimer::from_seconds(0.2),
    ));
}
