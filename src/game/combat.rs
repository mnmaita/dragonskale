use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::HALF_TILE_SIZE;

pub(super) struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnProjectileEvent>();

        app.add_systems(Update, spawn_projectiles);
    }
}

#[derive(Event)]
pub struct SpawnProjectileEvent {
    damage: i32,
    direction: Vec2,
    emitter: Entity,
    position: Vec2,
    speed: f32,
}

impl SpawnProjectileEvent {
    pub fn new(damage: i32, direction: Vec2, emitter: Entity, position: Vec2, speed: f32) -> Self {
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
pub struct ImpactDamage(pub i32);

/// Represents an Entity's damage attributes.
#[derive(Component)]
pub struct AttackDamage(pub i32);

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
