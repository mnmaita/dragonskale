use bevy::{prelude::*, render::view::RenderLayers};
use bevy_enoki::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioControl};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Sensor};

use crate::{
    audio::DragonBreathChannel,
    camera::{RenderLayer, YSorted},
    playing, AppState,
};

use super::{
    combat::ImpactDamage,
    resource_pool::{Fire, ResourcePool},
    Player, BUILDING_GROUP, ENEMY_GROUP, FIRE_BREATH_GROUP,
};

pub(super) struct FireBreathPlugin;

impl Plugin for FireBreathPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnFireBreathEvent>();

        app.add_plugins(EnokiPlugin);

        app.add_systems(
            FixedUpdate,
            (consume_fire_breath_resource, restore_fire_breath_resource).run_if(playing()),
        );

        app.add_systems(Update, spawn_fire_breath.run_if(playing()));

        let image = app
            .world()
            .resource::<AssetServer>()
            .load("textures/fire_anim.png");

        let sprite_particle_material = app
            .world_mut()
            .resource_mut::<Assets<SpriteParticle2dMaterial>>()
            .add(SpriteParticle2dMaterial::new(image, 2, 1));

        app.insert_resource(FireBreathParticleMaterialAsset(sprite_particle_material));
    }
}

#[derive(Deref, Resource)]
pub struct FireBreathParticleMaterialAsset(Handle<SpriteParticle2dMaterial>);

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

fn spawn_fire_breath(
    mut commands: Commands,
    mut spawn_fire_breath_event_reader: EventReader<SpawnFireBreathEvent>,
    material: Res<FireBreathParticleMaterialAsset>,
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
        commands.spawn((
            ParticleSpawner(material.clone()),
            ParticleEffectHandle(asset_server.load("vfx/fire_breath.ron")),
            OneShot::Despawn,
            Fire,
            RenderLayers::layer(RenderLayer::Ground.into()),
            Transform::from_translation(position.extend(10.0)),
            Sensor,
            Collider::ball(25.0),
            CollisionGroups::new(FIRE_BREATH_GROUP, BUILDING_GROUP | ENEMY_GROUP),
            StateScoped(AppState::GameOver),
            ImpactDamage(damage),
            YSorted,
        ));
    }
}

fn consume_fire_breath_resource(
    mut player_query: Query<&mut ResourcePool<Fire>, With<Player>>,
    spawn_fire_breath_event_reader: EventReader<SpawnFireBreathEvent>,
    dragon_breath_audio_channel: Res<AudioChannel<DragonBreathChannel>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    const FIRE_BREATH_CONSUMPTION_RATIO: i16 = 1;

    if !spawn_fire_breath_event_reader.is_empty() {
        let mut fire_resource_pool = player_query.single_mut();

        fire_resource_pool.subtract(FIRE_BREATH_CONSUMPTION_RATIO);

        if fire_resource_pool.is_empty() {
            audio.play(
                asset_server
                    .get_handle("sfx/breathend.ogg")
                    .unwrap_or_default(),
            );
            dragon_breath_audio_channel.stop();
        }
    }
}

fn restore_fire_breath_resource(
    mut player_query: Query<&mut ResourcePool<Fire>, With<Player>>,
    spawn_fire_breath_event_reader: EventReader<SpawnFireBreathEvent>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    const FIRE_BREATH_RESTORATION_RATIO: i16 = 1;

    if spawn_fire_breath_event_reader.is_empty() && !mouse_input.pressed(MouseButton::Left) {
        let mut fire_resource_pool = player_query.single_mut();

        fire_resource_pool.add(FIRE_BREATH_RESTORATION_RATIO);
    }
}
