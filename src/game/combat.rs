use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::{YSorted, SKY_LAYER},
    playing,
};

use super::{
    level::TileQuery,
    resource_pool::{Fire, Health, ResourcePool},
    score_system::{ScoreEvent, ScoreEventType},
    Enemy, InGameEntity, Player, Tile, HALF_TILE_SIZE, PLAYER_GROUP, PROJECTILE_GROUP,
};

pub(super) struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>();

        app.add_systems(
            FixedUpdate,
            (
                projectile_collision_with_player,
                spawn_projectiles,
                despawn_projectiles,
                compute_damage_from_intersections,
            )
                .run_if(playing()),
        );
    }
}

#[derive(Event)]
pub struct SpawnProjectileEvent {
    damage: i16,
    direction: Vec2,
    emitter: Entity,
    position: Vec2,
    speed: f32,
}

impl SpawnProjectileEvent {
    pub fn new(damage: i16, direction: Vec2, emitter: Entity, position: Vec2, speed: f32) -> Self {
        Self {
            damage,
            direction,
            emitter,
            position,
            speed,
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub ccd: Ccd,
    pub damage: ImpactDamage,
    pub emitter: Emitter,
    pub marker: Projectile,
    pub render_layers: RenderLayers,
    pub rigid_body: RigidBody,
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
}

#[derive(Component)]
pub struct Emitter(Entity);

#[derive(Component)]
pub struct Range(pub f32);

/// Represents the damage this entity causes to others when colliding.
#[derive(Component)]
pub struct ImpactDamage(pub i16);

/// Represents an Entity's damage attributes.
#[derive(Component)]
pub struct AttackDamage(pub i16);

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(Timer);

impl AttackTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct Projectile;

fn spawn_projectiles(
    mut commands: Commands,
    mut spawn_projectile_event_reader: EventReader<SpawnProjectileEvent>,
) {
    for &SpawnProjectileEvent {
        damage,
        direction,
        emitter,
        position,
        speed,
    } in spawn_projectile_event_reader.read()
    {
        let size = Vec2::new(HALF_TILE_SIZE.x, 3.);
        let angle = if direction != Vec2::ZERO {
            let mut angle = (direction).angle_between(Vec2::X);
            if !angle.is_finite() {
                angle = 0.;
            }
            angle
        } else {
            0.
        };

        let mut projectile_entity_commands = commands.spawn(ProjectileBundle {
            ccd: Ccd::enabled(),
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            collision_groups: CollisionGroups::new(
                PROJECTILE_GROUP,
                PLAYER_GROUP | PROJECTILE_GROUP,
            ),
            damage: ImpactDamage(damage),
            emitter: Emitter(emitter),
            marker: Projectile,
            render_layers: RenderLayers::layer(SKY_LAYER),
            rigid_body: RigidBody::Dynamic,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(position.extend(1.0))
                    .with_rotation(Quat::from_rotation_z(-angle)),
                ..default()
            },
            velocity: Velocity {
                linvel: direction * speed,
                angvel: 0.,
            },
        });

        projectile_entity_commands.insert((
            Damping {
                linear_damping: 1.0,
                ..default()
            },
            InGameEntity,
            YSorted,
        ));
    }
}

fn projectile_collision_with_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut ResourcePool<Health>), With<Player>>,
    mut score_event_writer: EventWriter<ScoreEvent>,
    projectile_query: Query<(Entity, &ImpactDamage), With<Projectile>>,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, mut player_hitpoints) = player_query.single_mut(); // A first entity with a collider attached.

    for (projectile_entity, projectile_damage) in &projectile_query {
        if let Some(contact_pair) = rapier_context.contact_pair(player_entity, projectile_entity) {
            if contact_pair.has_any_active_contacts() {
                player_hitpoints.subtract(projectile_damage.0);
                score_event_writer.send(ScoreEvent::new(0, ScoreEventType::ResetMultiplier));

                // TODO: Add "death" component or event and use it here so a different system handles despawns.
                commands.entity(projectile_entity).despawn_recursive();
            }
        }
    }
}

fn compute_damage_from_intersections(
    mut commands: Commands,
    fire_query: Query<(Entity, &ImpactDamage), With<Fire>>,
    mut enemy_query: Query<(Entity, &mut ResourcePool<Health>), With<Enemy>>,
    mut score_event_writer: EventWriter<ScoreEvent>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, damage) in &fire_query {
        for (entity1, entity2, intersecting) in rapier_context.intersections_with(entity) {
            let other_entity = if entity1 == entity { entity2 } else { entity1 };

            if intersecting {
                println!("INTERSECT");
                if let Ok((enemy_entity, mut enemy_hitpoints)) = enemy_query.get_mut(other_entity) {
                    enemy_hitpoints.subtract(damage.0);
                    commands.entity(enemy_entity).despawn_recursive();
                    score_event_writer.send(ScoreEvent::new(10, ScoreEventType::AddPoints));
                }
            }
        }
    }
}

pub fn despawn_projectiles(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Velocity), With<Projectile>>,
    tile_query: TileQuery,
) {
    for (entity, transform, velocity) in &projectile_query {
        if velocity.linvel.length() < 60. {
            if let Some(tile) = tile_query.get_from_position(transform.translation.truncate()) {
                if *tile == Tile::Water {
                    // TODO: Decouple this with a Despawn component
                    commands.entity(entity).despawn_recursive();
                } else {
                    // TODO: Add a DespawnTimer for arrows that land on the ground
                    commands.entity(entity).insert(ColliderDisabled);
                }
            }
        }
    }
}
