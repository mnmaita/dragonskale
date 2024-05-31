use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::TILE_SIZE;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            TILE_SIZE.y,
        ));

        #[cfg(debug_assertions)]
        app.add_plugins(RapierDebugRenderPlugin::default());

        let mut rapier_configuration = RapierConfiguration::new(1.0);

        rapier_configuration.gravity = Vec2::ZERO;
        rapier_configuration.timestep_mode = TimestepMode::Fixed {
            dt: 1. / 64.,
            substeps: 1,
        };

        app.insert_resource(rapier_configuration);
    }
}

#[derive(Component)]
pub struct Speed(pub f32);
