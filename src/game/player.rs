use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    camera::{RenderLayer, YSorted},
    physics::Speed,
    AppState,
};

use super::{
    resource_pool::{Fire, Health, ResourcePool},
    score_system::Score,
    PLAYER_GROUP, POWERUP_GROUP, PROJECTILE_GROUP,
};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}

#[derive(Component)]
#[require(
    AnimationIndices::new(0, 2),
    AnimationTimer::from_seconds(0.2),
    Collider::cuboid(15., 40.),
    CollisionGroups::new(PLAYER_GROUP, PROJECTILE_GROUP | POWERUP_GROUP),
    Speed(10.),
    ResourcePool::<Fire>::new(100),
    ResourcePool::<Health>::new(100),
    RenderLayers::layer(RenderLayer::Sky.into()),
    StateScoped::<AppState>(AppState::GameOver),
)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server
        .get_handle("textures/dragon.png")
        .unwrap_or_default();
    let texture_atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(191, 161), 12, 1, None, None);
    let texture_atlas_layout_handle = asset_server.add(texture_atlas_layout);

    commands.spawn((
        Player,
        // TODO: Score could be a bevy Resource
        Score::new(0, 1),
        Sprite {
            image,
            texture_atlas: Some(texture_atlas_layout_handle.into()),
            ..Default::default()
        },
        Transform::from_translation(Vec2::ONE.extend(1.)),
        YSorted,
    ));
}
