use bevy::{prelude::*, render::view::RenderLayers};
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Sensor};

use crate::{
    camera::{YSorted, SKY_LAYER},
    playing,
};

use super::{
    combat::ImpactDamage,
    resource_pool::{Fire, ResourcePool},
    InGameEntity, Player, BUILDING_GROUP, ENEMY_GROUP, FIRE_BREATH_GROUP,
};

pub(super) struct FireBreathPlugin;

impl Plugin for FireBreathPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnFireBreathEvent>();

        app.add_plugins(ParticleSystemPlugin);

        app.add_systems(
            FixedUpdate,
            (consume_fire_breath_resource, restore_fire_breath_resource).run_if(playing()),
        );

        app.add_systems(Update, spawn_fire_breath.run_if(playing()));
    }
}

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

#[derive(Bundle)]
struct FireBreathBundle {
    pub collider: Collider,
    pub damage: ImpactDamage,
    pub marker: Fire,
    pub particle_system: ParticleSystemBundle,
    pub render_layers: RenderLayers,
    pub sensor: Sensor,
}

fn spawn_fire_breath(
    mut commands: Commands,
    mut spawn_fire_breath_event_reader: EventReader<SpawnFireBreathEvent>,
    asset_server: Res<AssetServer>,
    player_query: Query<&ResourcePool<Fire>, With<Player>>,
) {
    let Ok(fire_resource_pool) = player_query.get_single() else {
        return;
    };

    if fire_resource_pool.is_empty() {
        return;
    }

    for &SpawnFireBreathEvent { damage, position } in spawn_fire_breath_event_reader.read() {
        let mut fire_breath_entity_commands = commands.spawn(FireBreathBundle {
            marker: Fire,
            particle_system: ParticleSystemBundle {
                transform: Transform::from_translation(position.extend(1.0)),
                particle_system: ParticleSystem {
                    z_value_override: Some(JitteredValue::new(0.9)),
                    max_particles: 10_000,
                    texture: ParticleTexture::Sprite(asset_server.load("textures/fire_breath.png")),
                    spawn_rate_per_second: 10.0.into(),
                    initial_speed: JitteredValue::jittered(3.0, -1.0..1.0),
                    lifetime: JitteredValue::jittered(4.0, -1.0..1.0),
                    looping: false,
                    despawn_on_finish: true,
                    system_duration_seconds: 1.0,
                    ..ParticleSystem::default()
                },
                ..ParticleSystemBundle::default()
            },
            render_layers: RenderLayers::layer(SKY_LAYER),
            sensor: Sensor,
            collider: Collider::ball(25.0),
            damage: ImpactDamage(damage),
        });

        fire_breath_entity_commands.insert((
            CollisionGroups::new(FIRE_BREATH_GROUP, BUILDING_GROUP | ENEMY_GROUP),
            InGameEntity,
            Playing,
            YSorted,
        ));
    }
}

fn consume_fire_breath_resource(
    mut player_query: Query<&mut ResourcePool<Fire>, With<Player>>,
    spawn_fire_breath_event_reader: EventReader<SpawnFireBreathEvent>,
) {
    const FIRE_BREATH_CONSUMPTION_RATIO: i16 = 1;

    if !spawn_fire_breath_event_reader.is_empty() {
        let mut fire_resource_pool = player_query.single_mut();

        fire_resource_pool.subtract(FIRE_BREATH_CONSUMPTION_RATIO);
    }
}

fn restore_fire_breath_resource(
    mut player_query: Query<&mut ResourcePool<Fire>, With<Player>>,
    spawn_fire_breath_event_reader: EventReader<SpawnFireBreathEvent>,
) {
    const FIRE_BREATH_RESTORATION_RATIO: i16 = 1;

    if spawn_fire_breath_event_reader.is_empty() {
        let mut fire_resource_pool = player_query.single_mut();

        fire_resource_pool.add(FIRE_BREATH_RESTORATION_RATIO);
    }
}
