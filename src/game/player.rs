use bevy::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    AppState,
};

use super::Hitpoints;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnFireBreathEvent>();
        app.add_plugins(ParticleSystemPlugin::default());
        app.add_systems(Update, spawn_fire_breath);
        app.add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub hitpoints: Hitpoints,
    pub marker: Player,
    pub spritesheet: SpriteSheetBundle,
}

#[derive(Component)]
pub struct Fire;

#[derive(Bundle)]
pub struct FireBreathBundle {
    pub particle_system: ParticleSystemBundle,
    pub damage: Damage,
    pub sensor: Sensor,
    pub collider: Collider,
    pub marker: Fire,
}

#[derive(Component)]
pub struct Damage(pub i16);

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct SpawnFireBreathEvent {
    damage: i16,
    position: Vec2,
}

impl SpawnFireBreathEvent {
    pub fn new(damage: i16, position: Vec2) -> Self {
        Self { damage, position }
    }
}

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

fn spawn_fire_breath(
    mut commands: Commands,
    mut spawn_fire_breath_event_reader: EventReader<SpawnFireBreathEvent>,
    asset_server: Res<AssetServer>,
) {
    for &SpawnFireBreathEvent { damage, position } in spawn_fire_breath_event_reader.read() {
        commands
            .spawn(FireBreathBundle {
                marker: Fire,
                particle_system: ParticleSystemBundle {
                    transform: Transform::from_translation(position.extend(1.0)),
                    particle_system: ParticleSystem {
                        z_value_override: Some(JitteredValue::new(0.9)), // temporary value 0.9 (under dragon), if set to 2, the fire is above
                        max_particles: 10_000,
                        texture: ParticleTexture::Sprite(
                            asset_server.load("textures/fire_breath.png"),
                        ),
                        spawn_rate_per_second: 10.0.into(),
                        initial_speed: JitteredValue::jittered(3.0, -1.0..1.0),
                        lifetime: JitteredValue::jittered(4.0, -1.0..1.0),
                        /* color: ColorOverTime::Gradient(Gradient::new(vec![
                            ColorPoint::new(Color::WHITE, 0.0),
                            ColorPoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
                        ])), */
                        looping: false,
                        despawn_on_finish: true,
                        system_duration_seconds: 1.0,
                        ..ParticleSystem::default()
                    },
                    ..ParticleSystemBundle::default()
                },
                sensor: Sensor,
                collider: Collider::ball(25.0),
                damage: Damage(damage),
            })
            // Add the playing component so it starts playing. This can be added later as well.
            .insert(Playing);
    }
}
