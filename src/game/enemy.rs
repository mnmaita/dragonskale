use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier2d::prelude::*;
use rand::{
    seq::{IndexedRandom as _, IteratorRandom},
    Rng,
};
use std::{collections::HashMap, time::Duration};

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    camera::{RenderLayer, YSorted},
    game::{combat::OnFire, game_timer::GameTimer},
    physics::Speed,
    playing, AppState,
};

use super::{
    combat::{AttackDamage, AttackTimer, Range, SpawnProjectileEvent},
    resource_pool::{Health, ResourcePool},
    BorderTile, Player, BUILDING_GROUP, ENEMY_GROUP, FIRE_BREATH_GROUP, HALF_TILE_SIZE, TILE_SIZE,
};

#[derive(Resource, Deref)]
struct AnimationTagMap(HashMap<AnimationTag, [usize; 2]>);

impl Default for AnimationTagMap {
    fn default() -> Self {
        Self(HashMap::from([
            (AnimationTag::RunLeft, [0, 7]),
            (AnimationTag::RunUpLeft, [16, 23]),
            (AnimationTag::RunUp, [32, 39]),
            (AnimationTag::RunUpRight, [48, 55]),
            (AnimationTag::RunRight, [64, 71]),
            (AnimationTag::RunDownRight, [80, 87]),
            (AnimationTag::RunDown, [96, 103]),
            (AnimationTag::RunDownLeft, [112, 119]),
            // TODO update all indexes
            (AnimationTag::AttackLeft, [12, 15]),
            (AnimationTag::AttackUpLeft, [28, 31]),
            (AnimationTag::AttackUp, [44, 47]),
            (AnimationTag::AttackUpRight, [60, 63]),
            (AnimationTag::AttackRight, [76, 79]),
            (AnimationTag::AttackDownRight, [92, 95]),
            (AnimationTag::AttackDown, [108, 111]),
            (AnimationTag::AttackDownLeft, [124, 127]),
        ]))
    }
}

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::new(3.));
        app.insert_resource(AnimationTagMap::default());

        app.add_systems(
            OnEnter(AppState::InGame),
            (load_atlas_handlers, setup_enemy_spawn_counter),
        );

        app.add_systems(
            FixedUpdate,
            (
                spawn_enemies,
                handle_enemy_movement,
                handle_enemy_attacks,
                update_enemy_animation_tag.after(handle_enemy_movement),
                update_enemy_animation_indexes.after(update_enemy_animation_tag),
            )
                .run_if(playing()),
        );

        app.add_observer(on_add_on_fire);
    }
}

#[derive(Component, Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum AnimationTag {
    RunLeft,
    RunUpLeft,
    RunUp,
    RunUpRight,
    RunRight,
    RunDownRight,
    RunDown,
    RunDownLeft,
    AttackLeft,
    AttackUpLeft,
    AttackUp,
    AttackUpRight,
    AttackRight,
    AttackDownRight,
    AttackDown,
    AttackDownLeft,
}

impl AnimationTag {
    pub fn current_attack_tag(dir: Dir2) -> Option<Self> {
        match dir {
            Dir2::NORTH => Some(Self::AttackUp),
            Dir2::EAST => Some(Self::AttackRight),
            Dir2::SOUTH => Some(Self::AttackDown),
            Dir2::WEST => Some(Self::AttackLeft),
            Dir2::NORTH_EAST => Some(Self::AttackUpRight),
            Dir2::NORTH_WEST => Some(Self::AttackUpLeft),
            Dir2::SOUTH_EAST => Some(Self::AttackDownRight),
            Dir2::SOUTH_WEST => Some(Self::AttackDownLeft),
            _ => None,
        }
    }

    pub fn current_run_tag(dir: Dir2) -> Option<Self> {
        match dir {
            Dir2::NORTH => Some(Self::RunUp),
            Dir2::EAST => Some(Self::RunRight),
            Dir2::SOUTH => Some(Self::RunDown),
            Dir2::WEST => Some(Self::RunLeft),
            Dir2::NORTH_EAST => Some(Self::RunUpRight),
            Dir2::NORTH_WEST => Some(Self::RunUpLeft),
            Dir2::SOUTH_EAST => Some(Self::RunDownRight),
            Dir2::SOUTH_WEST => Some(Self::RunDownLeft),
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
struct EnemySpawnCounter(u32);

#[derive(Resource, Deref, DerefMut)]
struct EnemySpawnTimer(Timer);

impl EnemySpawnTimer {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub enum Behavior {
    FollowPlayer { distance: f32 },
    Random,
}

#[derive(Component, Deref, DerefMut)]
pub struct FacingDirection(Dir2);

impl Default for FacingDirection {
    fn default() -> Self {
        Self(Dir2::WEST)
    }
}

#[derive(Resource)]
pub struct TextureArcherAtlasHandle(Handle<TextureAtlasLayout>);

#[derive(Resource)]
pub struct TextureAxeAtlasHandle(Handle<TextureAtlasLayout>);

fn load_atlas_handlers(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_atlas_archer = TextureAtlasLayout::from_grid(UVec2::new(72, 78), 16, 8, None, None);
    let texture_atlas_handle_archer = asset_server.add(texture_atlas_archer);

    commands.insert_resource(TextureArcherAtlasHandle(texture_atlas_handle_archer));

    let texture_atlas_axe = TextureAtlasLayout::from_grid(UVec2::new(72, 78), 16, 8, None, None);
    let texture_atlas_handle_axe = asset_server.add(texture_atlas_axe);

    commands.insert_resource(TextureAxeAtlasHandle(texture_atlas_handle_axe));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut enemy_spawn_counter: ResMut<EnemySpawnCounter>,
    tile_query: Query<&Transform, With<BorderTile>>,
    (texture_archer_atlas_handle, texture_axeman_atlas_handle): (
        Res<TextureArcherAtlasHandle>,
        Res<TextureAxeAtlasHandle>,
    ),
) {
    let duration = enemy_spawn_timer.duration();

    if enemy_spawn_timer.tick(time.delta()).just_finished() {
        enemy_spawn_counter.0 = enemy_spawn_counter.0.wrapping_add(1);

        if enemy_spawn_counter.0 % 10 == 0 {
            enemy_spawn_timer.set_duration(Duration::from_secs_f32(
                1.0_f32.max(duration.as_secs_f32() - 0.5),
            ));
        }

        let mut rng = rand::rng();
        if let Some(tile_transform) = tile_query.iter().choose(&mut rng) {
            let translation = tile_transform.translation.xy().extend(1.);

            //pick a random texture atlas handle between archer and axe
            let (texture_atlas_handle, image) = if rng.random_bool(0.5) {
                (
                    texture_archer_atlas_handle.0.clone(),
                    asset_server.load("textures/enemy_archer.png"),
                )
            } else {
                (
                    texture_axeman_atlas_handle.0.clone(),
                    asset_server.load("textures/enemy_axe.png"),
                )
            };

            commands
                .spawn((
                    Sprite {
                        image,
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_handle,
                            index: 4,
                        }),
                        ..Default::default()
                    },
                    Transform::from_translation(translation),
                    AttackDamage(5),
                    AttackTimer::new(3.),
                    Behavior::FollowPlayer {
                        distance: TILE_SIZE.x * 6.,
                    },
                    ResourcePool::<Health>::new(1),
                    Enemy,
                    Range(TILE_SIZE.x * 15.),
                    Speed(100.),
                    RenderLayers::layer(RenderLayer::Ground.into()),
                    FacingDirection::default(),
                    StateScoped(AppState::GameOver),
                    YSorted,
                ))
                .insert((
                    AnimationIndices::new(4, 11),
                    AnimationTimer::from_seconds(0.2),
                    AnimationTag::RunLeft,
                    Collider::cuboid(HALF_TILE_SIZE.x, HALF_TILE_SIZE.y),
                    RigidBody::Dynamic,
                    CollisionGroups::new(
                        ENEMY_GROUP,
                        ENEMY_GROUP | BUILDING_GROUP | FIRE_BREATH_GROUP,
                    ),
                    LockedAxes::ROTATION_LOCKED,
                ));
        }
    }
}

fn setup_enemy_spawn_counter(mut commands: Commands) {
    commands.insert_resource(EnemySpawnCounter(0));
}

fn update_enemy_animation_tag(
    mut enemy_query: Query<
        (&Transform, &Behavior, &FacingDirection, &mut AnimationTag),
        With<Enemy>,
    >,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_position = player_transform.translation.xy();

    for (enemy_transform, enemy_behavior, facing_direction, mut animation_tag) in &mut enemy_query {
        let current_tag = match enemy_behavior {
            Behavior::FollowPlayer { distance } => {
                if enemy_transform.translation.xy().distance(player_position) > *distance {
                    AnimationTag::current_run_tag(**facing_direction)
                } else {
                    AnimationTag::current_attack_tag(**facing_direction)
                }
            }
            Behavior::Random => AnimationTag::current_run_tag(**facing_direction),
        };

        if let Some(tag) = current_tag {
            *animation_tag = tag;
        }
    }
}

fn update_enemy_animation_indexes(
    mut enemy_query: Query<
        (&AnimationTag, &mut AnimationIndices, &mut Sprite),
        (With<Enemy>, Changed<AnimationTag>),
    >,
    animation_tag_map: Res<AnimationTagMap>,
) {
    for (animation_tag, mut animation_indices, mut sprite) in &mut enemy_query {
        if let Some(value) = animation_tag_map.get(animation_tag) {
            let [first, last] = *value;
            *animation_indices = AnimationIndices::new(first, last);
            if let Some(ref mut texture_atlas) = sprite.texture_atlas {
                texture_atlas.index = first;
            }
        };
    }
}

fn handle_enemy_movement(
    mut enemy_query: Query<
        (
            &mut Transform,
            &mut FacingDirection,
            Option<&mut GameTimer<Behavior>>,
            &Speed,
            &Behavior,
        ),
        With<Enemy>,
    >,
    player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();
    let player_position = player_transform.translation.xy();

    for (mut enemy_transform, mut facing_direction, behavior_timer, enemy_speed, enemy_behavior) in
        &mut enemy_query
    {
        match enemy_behavior {
            Behavior::FollowPlayer { distance } => {
                let enemy_position = enemy_transform.translation.xy();
                let new_direction = Dir2::new(player_position - enemy_position).unwrap_or(Dir2::X);

                if enemy_position.distance(player_position) > *distance {
                    enemy_transform.translation.x += new_direction.x * enemy_speed.0 * delta_secs;
                    enemy_transform.translation.y += new_direction.y * enemy_speed.0 * delta_secs;
                    **facing_direction = new_direction;
                }
            }
            Behavior::Random => {
                let should_change_direction = if let Some(mut timer) = behavior_timer {
                    timer.tick(time.delta()).just_finished()
                } else {
                    true
                };

                if should_change_direction {
                    let values = [0.0, 1.0];
                    let mut rng = rand::rng();
                    **facing_direction = Dir2::new(Vec2::new(
                        *values.choose(&mut rng).unwrap(),
                        *values.choose(&mut rng).unwrap(),
                    ))
                    .unwrap_or(Dir2::NEG_X);
                }

                enemy_transform.translation.x += facing_direction.x * enemy_speed.0 * delta_secs;
                enemy_transform.translation.y += facing_direction.y * enemy_speed.0 * delta_secs;
            }
        }
    }
}

fn handle_enemy_attacks(
    mut spawn_projectile_event_writer: EventWriter<SpawnProjectileEvent>,
    mut enemy_query: Query<
        (Entity, &Transform, &mut AttackTimer, &Range, &AttackDamage),
        (With<Enemy>, Without<OnFire>),
    >,
    player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_position = player_transform.translation.xy();

    for (enemy_entity, enemy_transform, mut enemy_attack_timer, enemy_range, enemy_attack_damage) in
        &mut enemy_query
    {
        if enemy_attack_timer.tick(time.delta()).just_finished() {
            let enemy_position = enemy_transform.translation.xy();

            if enemy_position.distance(player_position) <= enemy_range.0 {
                let direction = (player_position - enemy_position).normalize();
                let emitter = enemy_entity;

                spawn_projectile_event_writer.write(SpawnProjectileEvent::new(
                    enemy_attack_damage.0,
                    direction,
                    emitter,
                    enemy_position,
                    800.,
                ));
            }
        }
    }
}

fn on_add_on_fire(
    trigger: Trigger<OnAdd, OnFire>,
    mut query: Query<(Option<&mut Speed>, Option<&mut Behavior>)>,
    mut commands: Commands,
    mut texture_atlas_layout: Local<Handle<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let entity = trigger.target();

    *texture_atlas_layout = asset_server.add(TextureAtlasLayout::from_grid(
        UVec2::splat(40),
        2,
        1,
        None,
        None,
    ));

    if let Ok((speed, behavior)) = query.get_mut(entity) {
        if let Some(mut speed) = speed {
            **speed *= 2.0;
        }
        if let Some(mut behavior) = behavior {
            *behavior = Behavior::Random;
        }
        commands
            .entity(entity)
            .insert(GameTimer::<Behavior>::from_seconds(0.2))
            .with_child((
                Sprite {
                    color: Color::default().with_alpha(0.6),
                    image: asset_server.load("textures/fire_anim.png"),
                    texture_atlas: Some(TextureAtlas::from(texture_atlas_layout.clone())),
                    ..Default::default()
                },
                AnimationIndices::new(0, 1),
                AnimationTimer::from_seconds(0.2),
                RenderLayers::layer(RenderLayer::Ground.into()),
                StateScoped(AppState::GameOver),
            ));
    }
}
