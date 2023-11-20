use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier2d::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    camera::{YSorted, GROUND_LAYER},
    physics::Speed,
    playing,
};

use super::{
    combat::{AttackDamage, AttackTimer, Range, SpawnProjectileEvent},
    BorderTile, Hitpoints, Player, HALF_TILE_SIZE, TILE_SIZE,
};

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::new(3.));

        app.add_systems(
            FixedUpdate,
            (spawn_enemies, handle_enemy_behavior, handle_enemy_attacks).run_if(playing()),
        );
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub attack_damage: AttackDamage,
    pub attack_timer: AttackTimer,
    pub behavior: Behavior,
    pub hitpoints: Hitpoints,
    pub marker: Enemy,
    pub range: Range,
    pub speed: Speed,
    pub sprite: SpriteBundle,
    pub collider: Collider,
    pub render_layers: RenderLayers,
    pub rigid_body: RigidBody,
    pub collision_groups: CollisionGroups,
}

#[derive(Component)]
pub struct Enemy;

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

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    tile_query: Query<&Transform, With<BorderTile>>,
) {
    if enemy_spawn_timer.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        if let Some(tile_transform) = tile_query.iter().choose(&mut rng) {
            let translation = tile_transform.translation.truncate().extend(1.);
            let mut enemy_entity_commands = commands.spawn(EnemyBundle {
                attack_damage: AttackDamage(5),
                attack_timer: AttackTimer(Timer::from_seconds(5., TimerMode::Repeating)),
                behavior: Behavior::FollowPlayer {
                    distance: TILE_SIZE.x * 6.,
                },
                hitpoints: Hitpoints::new(1),
                marker: Enemy,
                range: Range(TILE_SIZE.x * 12.),
                speed: Speed(2.),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(TILE_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(translation),
                    ..default()
                },
                collider: Collider::cuboid(HALF_TILE_SIZE.x, HALF_TILE_SIZE.y),
                render_layers: RenderLayers::layer(GROUND_LAYER),
                rigid_body: RigidBody::Dynamic,
                collision_groups: CollisionGroups::new(Group::GROUP_2, Group::GROUP_2),
            });

            enemy_entity_commands.insert((LockedAxes::ROTATION_LOCKED, YSorted));
        }
    }
}

fn handle_enemy_behavior(
    mut enemy_query: Query<(&mut Transform, &Speed, &Behavior), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    let player_position = player_transform.translation.truncate();

    for (mut enemy_transform, enemy_speed, enemy_behavior) in &mut enemy_query {
        match enemy_behavior {
            &Behavior::FollowPlayer { distance } => {
                let enemy_position = enemy_transform.translation.truncate();
                if enemy_position.distance(player_position) > distance {
                    let enemy_direction = (player_position - enemy_position).normalize();
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
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let player_position = player_transform.translation.truncate();

    for (enemy_entity, enemy_transform, mut enemy_attack_timer, enemy_range, enemy_attack_damage) in
        &mut enemy_query
    {
        if enemy_attack_timer.tick(time.delta()).just_finished() {
            let enemy_position = enemy_transform.translation.truncate();

            if enemy_position.distance(player_position) <= enemy_range.0 {
                let direction = (player_position - enemy_position).normalize();
                let emitter = enemy_entity;

                spawn_projectile_event_writer.send(SpawnProjectileEvent::new(
                    enemy_attack_damage.0,
                    direction,
                    emitter,
                    enemy_position,
                    1000.,
                ))
            }
        }
    }
}
