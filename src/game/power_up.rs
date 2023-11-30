use bevy::{
    app::Update,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_rapier2d::{
    dynamics::{LockedAxes, RigidBody},
    geometry::{Collider, CollisionGroups},
};
use rand::Rng;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    camera::{RenderLayer, YSorted},
    playing,
};

use super::{plugin::InGameEntity, BUILDING_GROUP, ENEMY_GROUP, FIRE_BREATH_GROUP, HALF_TILE_SIZE};

pub(super) struct PowerUpSystemPlugin;

impl Plugin for PowerUpSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PowerUpEvent>();
        app.add_systems(Update, spawn_powerups.run_if(playing()));
    }
}

//event and event type
#[derive(Event)]
pub struct PowerUpEvent {
    transform: Transform,
    powerup_event_type: PowerUpEventType,
}
#[derive(Component)]
pub enum PowerUpEventType {
    HealingScale,
}

impl PowerUpEvent {
    pub fn new(transform: Transform, powerup_event_type: PowerUpEventType) -> Self {
        Self {
            transform,
            powerup_event_type,
        }
    }
}

#[derive(Bundle)]
pub struct PowerUpBundle {
    pub marker: PowerUp,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub sprite: SpriteSheetBundle,
    pub collider: Collider,
    pub render_layers: RenderLayers,
    pub rigid_body: RigidBody,
    pub collision_groups: CollisionGroups,
}

#[derive(Component)]
pub struct PowerUp;

fn spawn_powerups(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut powerup_event_reader: EventReader<PowerUpEvent>,
) {
    for PowerUpEvent {
        transform,
        powerup_event_type,
    } in powerup_event_reader.read()
    {
        match powerup_event_type {
            PowerUpEventType::HealingScale => {
                // spawn powerup entity here with transform

                let texture_healing_scale = asset_server
                    .get_handle("textures/scale_anim.png")
                    .unwrap_or_default();

                let texture_atlas_healing_scale = TextureAtlas::from_grid(
                    texture_healing_scale,
                    Vec2::new(40., 40.),
                    2,
                    1,
                    None,
                    None,
                );
                let texture_atlas_handle_healing_scale =
                    asset_server.add(texture_atlas_healing_scale);

                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    // spawneo
                    let mut powerup_entity_commands = commands.spawn(PowerUpBundle {
                        marker: PowerUp,
                        animation_indices: AnimationIndices::new(0, 1),
                        animation_timer: AnimationTimer::from_seconds(0.2),
                        sprite: SpriteSheetBundle {
                            sprite: TextureAtlasSprite::new(0),
                            texture_atlas: texture_atlas_handle_healing_scale.clone(),
                            transform: *transform,
                            ..default()
                        },
                        collider: Collider::cuboid(HALF_TILE_SIZE.x, HALF_TILE_SIZE.y),
                        render_layers: RenderLayers::layer(RenderLayer::Ground.into()),
                        rigid_body: RigidBody::Dynamic,
                        collision_groups: CollisionGroups::new(
                            ENEMY_GROUP,
                            ENEMY_GROUP | BUILDING_GROUP | FIRE_BREATH_GROUP,
                        ),
                    });
                    powerup_entity_commands.insert((
                        InGameEntity,
                        LockedAxes::ROTATION_LOCKED,
                        YSorted,
                    ));
                };
            }
        }
    }
}
/*
fn consume_powerups(
    mut score_event_reader: EventReader<ScoreEvent>,
    mut player_score_query: Query<&mut Score, With<Player>>,
)
 */
