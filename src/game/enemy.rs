use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier2d::prelude::*;
use rand::{seq::IteratorRandom, Rng};
use std::{collections::HashMap, time::Duration};

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    camera::{RenderLayer, YSorted},
    physics::Speed,
    playing, AppState,
};

use super::{
    combat::{AttackDamage, AttackTimer, Range, SpawnProjectileEvent},
    resource_pool::{Health, ResourcePool},
    BorderTile, Player, BUILDING_GROUP, ENEMY_GROUP, FIRE_BREATH_GROUP, HALF_TILE_SIZE, TILE_SIZE,
};

#[derive(Resource, Deref)]
struct SpriteAnimationMap(HashMap<SpriteAnimation, [usize; 2]>);

impl Default for SpriteAnimationMap {
    fn default() -> Self {
        Self(HashMap::from([
            (SpriteAnimation::RunLeft, [0, 7]),
            (SpriteAnimation::RunUpLeft, [16, 23]),
            (SpriteAnimation::RunUp, [32, 39]),
            (SpriteAnimation::RunUpRight, [48, 55]),
            (SpriteAnimation::RunRight, [64, 71]),
            (SpriteAnimation::RunDownRight, [80, 87]),
            (SpriteAnimation::RunDown, [96, 103]),
            (SpriteAnimation::RunDownLeft, [112, 119]),
            // TODO update all indexes
            (SpriteAnimation::AttackLeft, [12, 15]),
            (SpriteAnimation::AttackUpLeft, [28, 31]),
            (SpriteAnimation::AttackUp, [44, 47]),
            (SpriteAnimation::AttackUpRight, [60, 63]),
            (SpriteAnimation::AttackRight, [76, 79]),
            (SpriteAnimation::AttackDownRight, [92, 95]),
            (SpriteAnimation::AttackDown, [108, 111]),
            (SpriteAnimation::AttackDownLeft, [124, 127]),
        ]))
    }
}

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::new(3.));
        app.insert_resource(SpriteAnimationMap::default());

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
                update_enemy_facing_direction.after(handle_enemy_movement),
                update_enemy_sprite_animation.after(update_enemy_facing_direction),
                update_enemy_animation_indexes.after(update_enemy_sprite_animation),
            )
                .run_if(playing()),
        );
    }
}

#[derive(Component, Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum SpriteAnimation {
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
}

#[derive(Component, Deref, DerefMut)]
pub struct FacingDirection(Dir2);

impl Default for FacingDirection {
    fn default() -> Self {
        Self(Dir2::NEG_X)
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
            let translation = tile_transform.translation.truncate().extend(1.);

            //pick a random texture atlas handle between archer and axe
            let (texture_atlas_handle, image) = if rng.random_bool(0.5) {
                (
                    texture_archer_atlas_handle.0.clone(),
                    asset_server
                        .get_handle("textures/enemy_archer.png")
                        .unwrap_or_default(),
                )
            } else {
                (
                    texture_axeman_atlas_handle.0.clone(),
                    asset_server
                        .get_handle("textures/enemy_axe.png")
                        .unwrap_or_default(),
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
                    Speed(2.),
                    RenderLayers::layer(RenderLayer::Ground.into()),
                    FacingDirection::default(),
                    StateScoped(AppState::GameOver),
                    YSorted,
                ))
                .insert((
                    AnimationIndices::new(4, 11),
                    AnimationTimer::from_seconds(0.2),
                    SpriteAnimation::RunLeft,
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

fn update_enemy_facing_direction(
    mut enemy_query: Query<(&Transform, &Behavior, &mut FacingDirection), With<Enemy>>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_position = player_transform.translation.truncate();

    for (enemy_transform, enemy_behavior, mut enemy_facing_direction) in &mut enemy_query {
        match enemy_behavior {
            &Behavior::FollowPlayer { distance } => {
                let enemy_position = enemy_transform.translation.truncate();

                if enemy_position.distance(player_position) > distance {
                    enemy_facing_direction.0 =
                        Dir2::new(player_position - enemy_position).unwrap_or(Dir2::NEG_X);
                }
            }
        }
    }
}

fn update_enemy_sprite_animation(
    mut enemy_query: Query<
        (
            &Transform,
            &Behavior,
            &FacingDirection,
            &mut SpriteAnimation,
        ),
        With<Enemy>,
    >,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_position = player_transform.translation.truncate();

    for (enemy_transform, enemy_behavior, facing_direction, mut sprite_animation) in
        &mut enemy_query
    {
        match enemy_behavior {
            &Behavior::FollowPlayer { distance } => {
                let enemy_position = enemy_transform.translation.truncate();

                if enemy_position.distance(player_position) > distance {
                    match facing_direction.0 {
                        Dir2::NORTH => *sprite_animation = SpriteAnimation::RunUp,
                        Dir2::EAST => *sprite_animation = SpriteAnimation::RunRight,
                        Dir2::SOUTH => *sprite_animation = SpriteAnimation::RunDown,
                        Dir2::WEST => *sprite_animation = SpriteAnimation::RunLeft,
                        Dir2::NORTH_EAST => *sprite_animation = SpriteAnimation::RunUpRight,
                        Dir2::NORTH_WEST => *sprite_animation = SpriteAnimation::RunUpLeft,
                        Dir2::SOUTH_EAST => *sprite_animation = SpriteAnimation::RunDownRight,
                        Dir2::SOUTH_WEST => *sprite_animation = SpriteAnimation::RunDownLeft,
                        _ => (),
                    }
                } else {
                    match facing_direction.0 {
                        Dir2::NORTH => *sprite_animation = SpriteAnimation::AttackUp,
                        Dir2::EAST => *sprite_animation = SpriteAnimation::AttackRight,
                        Dir2::SOUTH => *sprite_animation = SpriteAnimation::AttackDown,
                        Dir2::WEST => *sprite_animation = SpriteAnimation::AttackLeft,
                        Dir2::NORTH_EAST => *sprite_animation = SpriteAnimation::AttackUpRight,
                        Dir2::NORTH_WEST => *sprite_animation = SpriteAnimation::AttackUpLeft,
                        Dir2::SOUTH_EAST => *sprite_animation = SpriteAnimation::AttackDownRight,
                        Dir2::SOUTH_WEST => *sprite_animation = SpriteAnimation::AttackDownLeft,
                        _ => (),
                    }
                }
            }
        }
    }
}

fn update_enemy_animation_indexes(
    mut enemy_query: Query<
        (&SpriteAnimation, &mut AnimationIndices, &mut Sprite),
        (With<Enemy>, Changed<SpriteAnimation>),
    >,
    sprite_animation_map: Res<SpriteAnimationMap>,
) {
    for (sprite_animation, mut animation_indices, mut sprite) in &mut enemy_query {
        if let Some(value) = sprite_animation_map.get(sprite_animation) {
            let [first, last] = *value;
            *animation_indices = AnimationIndices::new(first, last);
            if let Some(ref mut texture_atlas) = sprite.texture_atlas {
                texture_atlas.index = first;
            }
        };
    }
}

fn handle_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Speed, &Behavior), With<Enemy>>,
    player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_position = player_transform.translation.truncate();

    for (mut enemy_transform, enemy_speed, enemy_behavior) in &mut enemy_query {
        match enemy_behavior {
            &Behavior::FollowPlayer { distance } => {
                let enemy_position = enemy_transform.translation.truncate();

                let enemy_direction =
                    Dir2::new(player_position - enemy_position).unwrap_or(Dir2::X);

                if enemy_position.distance(player_position) > distance {
                    enemy_transform.translation.x += enemy_direction.x * enemy_speed.0;
                    enemy_transform.translation.y += enemy_direction.y * enemy_speed.0;
                }
            }
        }
    }
}

fn handle_enemy_attacks(
    mut spawn_projectile_event_writer: EventWriter<SpawnProjectileEvent>,
    mut enemy_query: Query<
        (Entity, &Transform, &mut AttackTimer, &Range, &AttackDamage),
        With<Enemy>,
    >,
    player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_position = player_transform.translation.truncate();

    for (enemy_entity, enemy_transform, mut enemy_attack_timer, enemy_range, enemy_attack_damage) in
        &mut enemy_query
    {
        if enemy_attack_timer.tick(time.delta()).just_finished() {
            let enemy_position = enemy_transform.translation.truncate();

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
