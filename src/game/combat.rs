use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Hitpoints, Player, HALF_TILE_SIZE};

pub(super) struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>();

        app.add_systems(Update, spawn_projectiles);

        app.add_systems(FixedUpdate, projectile_collision_with_player);
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
    pub ccd: Ccd,
    pub damage: ImpactDamage,
    pub emitter: Emitter,
    pub marker: Projectile,
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
pub struct AttackTimer(pub Timer);

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

        commands.spawn(ProjectileBundle {
            ccd: Ccd::enabled(),
            collider: Collider::cuboid(size.x, size.y),
            damage: ImpactDamage(damage),
            emitter: Emitter(emitter),
            marker: Projectile,
            rigid_body: RigidBody::Dynamic,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(size),
                    ..default()
                },
                // FIXME: Add rotation to the projectile so it faces towards its direction.
                transform: Transform::from_translation(position.extend(1.0)),
                ..default()
            },
            velocity: Velocity {
                linvel: direction * speed,
                angvel: 0.,
            },
        });
    }
}

fn projectile_collision_with_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Hitpoints), With<Player>>,
    projectile_query: Query<(Entity, &ImpactDamage), With<Projectile>>,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, mut player_hitpoints) = player_query.single_mut(); // A first entity with a collider attached.

    for (projectile_entity, projectile_damage) in &projectile_query {
        if let Some(contact_pair) = rapier_context.contact_pair(player_entity, projectile_entity) {
            if contact_pair.has_any_active_contacts() {
                player_hitpoints.subtract(projectile_damage.0);
                // TODO: Add "death" component or event and use it here so a different system handles despawns.
                commands.entity(projectile_entity).despawn_recursive();
            }
        }
    }
}
