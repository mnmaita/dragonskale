use bevy::prelude::*;

use crate::{
    game::Player,
    game::{GRID_SIZE, HALF_TILE_SIZE, TILE_SIZE},
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
        let viewport_size_remainder = viewport_size % TILE_SIZE;
        let camera_boundary_size = (level_dimensions
            - (viewport_size - viewport_size_remainder)
            - viewport_size_remainder)
            .clamp(Vec2::ZERO, Vec2::splat(f32::MAX));
        let camera_boundary = Rect::from_center_size(-HALF_TILE_SIZE, camera_boundary_size);

        if camera_boundary.is_empty() {
            if viewport_size.x > level_dimensions.x {
                camera_transform.translation.x = 0.0;
            }
            if viewport_size.y > level_dimensions.y {
                camera_transform.translation.y = 0.0;
            }
        }

        if camera_boundary.size() != Vec2::ZERO
            && !camera_boundary.contains(camera_transform.translation.truncate())
        {
            let (min_x, max_x) = (camera_boundary.min.x, camera_boundary.max.x);
            let (min_y, max_y) = (camera_boundary.min.y, camera_boundary.max.y);
            camera_transform.translation.x = camera_transform.translation.x.clamp(min_x, max_x);
            camera_transform.translation.y = camera_transform.translation.y.clamp(min_y, max_y);
        }
    }
}
