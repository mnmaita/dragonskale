use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::TILE_SIZE;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            TILE_SIZE.y,
        ));

        app.insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            timestep_mode: TimestepMode::Fixed {
                dt: 1. / 64.,
                substeps: 1,
            },
            ..default()
        });
    }
}

#[derive(Component)]
pub struct Speed(pub f32);
