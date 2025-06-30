use bevy::{prelude::*, render::view::RenderLayers};
use bevy_enhanced_input::prelude::*;
use bevy_enoki::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioControl};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Sensor};

use crate::{
    animation::AnimationIndices,
    audio::DragonBreathChannel,
    camera::{RenderLayer, YSorted},
    input::{actions::FireBreath, DefaultInputContext},
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
        app.add_event::<SubtractFireResourceEvent>();

        app.add_systems(
            FixedUpdate,
            (
                consume_fire_breath_resource.run_if(on_event::<SubtractFireResourceEvent>),
                restore_fire_breath_resource,
            )
                .run_if(playing()),
        );

        app.add_systems(Update, play_breath_end_sfx.run_if(playing()));

        let image = app
            .world()
            .resource::<AssetServer>()
            .load("textures/fire_anim.png");

        let sprite_particle_material = app
            .world_mut()
            .resource_mut::<Assets<SpriteParticle2dMaterial>>()
            .add(SpriteParticle2dMaterial::new(image, 2, 1));

        app.insert_resource(FireBreathParticleMaterialAsset(sprite_particle_material));

        app.add_observer(spawn_fire_breath);
        app.add_observer(on_fire_breath_started);
        app.add_observer(on_fire_breath_ongoing);
        app.add_observer(on_fire_breath_fired);
        app.add_observer(on_fire_breath_canceled);
        app.add_observer(on_fire_breath_completed);
    }
}

#[derive(Event)]
pub struct SubtractFireResourceEvent;

#[derive(Deref, Resource)]
pub struct FireBreathParticleMaterialAsset(Handle<SpriteParticle2dMaterial>);

fn spawn_fire_breath(
    _trigger: Trigger<Fired<FireBreath>>,
    mut commands: Commands,
    material: Res<FireBreathParticleMaterialAsset>,
    asset_server: Res<AssetServer>,
    player: Single<(&Transform, &ResourcePool<Fire>), With<Player>>,
) {
    let (player_transform, fire_resource_pool) = player.into_inner();

    if fire_resource_pool.is_empty() {
        return;
    }

    let player_direction = player_transform.rotation.mul_vec3(Vec3::Y).xy();
    // TODO: replace literal value with player sprite dimensions
    let fire_position = player_transform.translation.xy() + player_direction * 90.;
    let damage = 1;

    commands.spawn((
        ParticleSpawner(material.clone()),
        ParticleEffectHandle(asset_server.load("vfx/fire_breath.ron")),
        OneShot::Despawn,
        Fire,
        RenderLayers::layer(RenderLayer::Ground.into()),
        Transform::from_translation(fire_position.extend(10.0)),
        Sensor,
        Collider::ball(25.0),
        CollisionGroups::new(FIRE_BREATH_GROUP, BUILDING_GROUP | ENEMY_GROUP),
        StateScoped(AppState::GameOver),
        ImpactDamage(damage),
        YSorted,
    ));
}

fn consume_fire_breath_resource(
    mut fire_resource_pool: Single<&mut ResourcePool<Fire>, With<Player>>,
) {
    const FIRE_BREATH_CONSUMPTION_RATIO: i16 = 1;
    fire_resource_pool.subtract(FIRE_BREATH_CONSUMPTION_RATIO);
}

fn restore_fire_breath_resource(
    player: Single<(&Actions<DefaultInputContext>, &mut ResourcePool<Fire>), With<Player>>,
) -> Result<()> {
    const FIRE_BREATH_RESTORATION_RATIO: i16 = 1;

    let (actions, mut fire_resource_pool) = player.into_inner();

    if actions.state::<FireBreath>()? == ActionState::None {
        fire_resource_pool.add(FIRE_BREATH_RESTORATION_RATIO);
    }

    Ok(())
}

fn on_fire_breath_started(
    _trigger: Trigger<Started<FireBreath>>,
    dragon_breath_audio_channel: Res<AudioChannel<DragonBreathChannel>>,
    asset_server: Res<AssetServer>,
) {
    dragon_breath_audio_channel.play(
        asset_server
            .get_handle("sfx/breathstart.ogg")
            .unwrap_or_default(),
    );
}

fn on_fire_breath_ongoing(
    _trigger: Trigger<Ongoing<FireBreath>>,
    mut player_animation_indices: Single<&mut AnimationIndices, With<Player>>,
) {
    **player_animation_indices = AnimationIndices::new(6, 8);
}

fn on_fire_breath_fired(
    trigger: Trigger<Fired<FireBreath>>,
    mut subtract_fire_resource_event_writer: EventWriter<SubtractFireResourceEvent>,
    player: Single<(&mut AnimationIndices, &ResourcePool<Fire>), With<Player>>,
    dragon_breath_audio_channel: Res<AudioChannel<DragonBreathChannel>>,
    asset_server: Res<AssetServer>,
) {
    let (mut player_animation_indices, fire_resource_pool) = player.into_inner();

    if !fire_resource_pool.is_empty() {
        subtract_fire_resource_event_writer.write(SubtractFireResourceEvent);
    }

    if trigger.fired_secs == 0.0 {
        *player_animation_indices = Player::default_animation_indices();
        dragon_breath_audio_channel
            .play(
                asset_server
                    .get_handle("sfx/breathloop.ogg")
                    .unwrap_or_default(),
            )
            .looped();
    }
}

fn on_fire_breath_canceled(
    _trigger: Trigger<Canceled<FireBreath>>,
    mut player_animation_indices: Single<&mut AnimationIndices, With<Player>>,
    dragon_breath_audio_channel: Res<AudioChannel<DragonBreathChannel>>,
) {
    **player_animation_indices = Player::default_animation_indices();
    dragon_breath_audio_channel.stop();
}

fn on_fire_breath_completed(
    _trigger: Trigger<Completed<FireBreath>>,
    fire_resource_pool: Single<&ResourcePool<Fire>, With<Player>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    dragon_breath_audio_channel: Res<AudioChannel<DragonBreathChannel>>,
) {
    if !fire_resource_pool.is_empty() {
        audio.play(
            asset_server
                .get_handle("sfx/breathend.ogg")
                .unwrap_or_default(),
        );
        dragon_breath_audio_channel.stop();
    }
}

fn play_breath_end_sfx(
    fire_resource_pool: Single<&ResourcePool<Fire>, (With<Player>, Changed<ResourcePool<Fire>>)>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    dragon_breath_audio_channel: Res<AudioChannel<DragonBreathChannel>>,
) {
    if fire_resource_pool.is_empty() {
        audio.play(
            asset_server
                .get_handle("sfx/breathend.ogg")
                .unwrap_or_default(),
        );
        dragon_breath_audio_channel.stop();
    }
}
