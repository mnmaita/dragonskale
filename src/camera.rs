use bevy::prelude::*;

use crate::{
    game::Player,
    level::{GRID_SIZE, TILE_SIZE},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);

        app.add_systems(
            Update,
            (
                update_camera.run_if(any_with_component::<Player>()),
                constrain_camera_position_to_level,
            )
                .chain(),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn constrain_camera_position_to_level(
    mut camera_query: Query<(&Camera, &mut Transform), With<Camera2d>>,
) {
    let (camera, mut camera_transform) = camera_query.single_mut();

    if let Some(viewport_size) = camera.logical_viewport_size() {
        let level_dimensions = GRID_SIZE * TILE_SIZE;
        let half_viewport_size = viewport_size * 0.5;
        let half_tile_size = TILE_SIZE * 0.5;

        camera_transform.translation.x = if viewport_size.x > level_dimensions.x {
            0.0
        } else {
            let level_half_width = level_dimensions.x * 0.5;
            let min = -level_half_width + half_viewport_size.x + half_tile_size.x;
            let max = level_half_width - half_viewport_size.x - half_tile_size.x;
            camera_transform.translation.x.clamp(min, max)
        };

        camera_transform.translation.y = if viewport_size.y > level_dimensions.y {
            0.0
        } else {
            let level_half_height = level_dimensions.y * 0.5;
            let min = -level_half_height + half_viewport_size.y + half_tile_size.y;
            let max = level_half_height - half_viewport_size.y - half_tile_size.y;
            camera_transform.translation.y.clamp(min, max)
        };
    }
}
