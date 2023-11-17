use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    physics::Speed,
    playing, AppState,
};

use super::Hitpoints;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovementEvent>();

        app.add_systems(OnEnter(AppState::InGame), spawn_player);

        app.add_systems(FixedUpdate, handle_player_movement_events.run_if(playing()));
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub collider: Collider,
    pub damping: Damping,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub hitpoints: Hitpoints,
    pub marker: Player,
    pub rigid_body: RigidBody,
    pub speed: Speed,
    pub spritesheet: SpriteSheetBundle,
}

#[derive(Component)]
pub struct Player;

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
        damping: Damping::default(),
        external_force: ExternalForce::default(),
        external_impulse: ExternalImpulse::default(),
        hitpoints: Hitpoints::new(100),
        marker: Player,
        rigid_body: RigidBody::Dynamic,
        speed: Speed(15.),
        spritesheet: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec2::ZERO.extend(1.)),
            ..default()
        },
    });
}

#[derive(Event)]
pub enum PlayerMovementEvent {
    Accelerate { target: Vec2 },
    Brake,
}

impl PlayerMovementEvent {
    pub fn accelerate(target: Vec2) -> Self {
        Self::Accelerate { target }
    }

    pub fn brake() -> Self {
        Self::Brake
    }
}

fn handle_player_movement_events(
    mut player_movement_event_reader: EventReader<PlayerMovementEvent>,
    mut query: Query<
        (
            &Transform,
            &mut ExternalForce,
            &mut ExternalImpulse,
            &mut Damping,
        ),
        With<Player>,
    >,
) {
    let (transform, mut external_force, mut external_impulse, mut damping) = query.single_mut();

    for event in player_movement_event_reader.read() {
        match event {
            &PlayerMovementEvent::Accelerate { target } => {
                let player_position = transform.translation.truncate();
                let target_to_player_vector = target - player_position;

                if target_to_player_vector == Vec2::ZERO {
                    continue;
                }

                let target_distance_to_player = target.distance(player_position);
                let velocity_scalar = target_distance_to_player.min(300.) / 300.;
                let direction = transform.rotation.mul_vec3(Vec3::Y).truncate();
                let angle_with_cursor =
                    direction.angle_between(target_to_player_vector.normalize());
                let is_in_cruise_mode = (-0.4..0.4).contains(&angle_with_cursor);
                let angle = target_to_player_vector.angle_between(direction);

                *damping = Damping::default();
                external_impulse.impulse = direction * velocity_scalar * 125.;

                let (torque, strafe) = {
                    if is_in_cruise_mode {
                        external_impulse.impulse *= 2.;
                        damping.angular_damping = 10.;
                        (0., Vec2::ZERO)
                    } else {
                        (-angle * 75., direction.perp() * -angle * 31250.)
                    }
                };

                external_impulse.torque_impulse = torque;
                external_force.force = strafe;
            }
            PlayerMovementEvent::Brake => {
                *damping = Damping {
                    angular_damping: 25.,
                    linear_damping: 25.,
                };
            }
        }
    }
}
