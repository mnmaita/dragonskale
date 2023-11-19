use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    AppState,
};

use super::{fire_breath::Fire, resource_pool::ResourcePool, Hitpoints};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub fire_breath_resource: ResourcePool<Fire>,
    pub hitpoints: Hitpoints,
    pub marker: Player,
    pub spritesheet: SpriteSheetBundle,
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server
        .get_handle("textures/dragon.png")
        .unwrap_or_default();
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(191., 161.), 12, 1, None, None);
    let texture_atlas_handle = asset_server.add(texture_atlas);

    commands.spawn(PlayerBundle {
        animation_indices: AnimationIndices::new(0, 2),
        animation_timer: AnimationTimer::from_seconds(0.2),
        collider: Collider::ball(80.5),
        collision_groups: CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
        fire_breath_resource: ResourcePool::<Fire>::new(100),
        hitpoints: Hitpoints::new(100),
        marker: Player,
        spritesheet: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec2::ZERO.extend(1.)),
            ..default()
        },
    });
}
