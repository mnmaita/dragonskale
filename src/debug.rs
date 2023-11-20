use bevy::prelude::*;

use crate::{
    game::{BorderTile, Player, Tile, GRID_SIZE, HALF_TILE_SIZE, TILE_SIZE},
    input::CursorWorldPositionChecker,
    playing,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                debug_draw_tiles,
                draw_camera_constraints,
                draw_mouse_direction,
            )
                .run_if(playing()),
        );
    }
}

fn debug_draw_tiles(
    query: Query<(&Transform, Option<&BorderTile>), With<Tile>>,
    mut gizmos: Gizmos,
) {
    for (transform, border_tile) in &query {
        gizmos.rect_2d(
            transform.translation.truncate(),
            0.,
            TILE_SIZE,
            if border_tile.is_some() {
                Color::BLACK
            } else {
                Color::FUCHSIA
            },
        );
    }
}

fn draw_camera_constraints(
    camera_query: Query<(&Camera, &Transform), With<Camera2d>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    if let Some(viewport_size) = camera.logical_viewport_size() {
        let level_dimensions = GRID_SIZE * TILE_SIZE;
        let viewport_size_remainder = viewport_size % TILE_SIZE;
        let camera_boundary_size = (level_dimensions
            - (viewport_size - viewport_size_remainder)
            - viewport_size_remainder)
            .clamp(Vec2::ZERO, Vec2::splat(f32::MAX));
        let camera_boundary = Rect::from_center_size(-HALF_TILE_SIZE, camera_boundary_size);

        gizmos.rect_2d(
            camera_boundary.center(),
            0.,
            camera_boundary.size(),
            Color::RED,
        );
        gizmos.circle_2d(camera_transform.translation.truncate(), 10., Color::RED);
    }
}

fn draw_mouse_direction(
    mouse_input: ResMut<Input<MouseButton>>,
    cursor_world_position_checker: CursorWorldPositionChecker,
    query: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
) {
    if mouse_input.pressed(MouseButton::Right) {
        if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
            let player_transform = query.single();
            let player_position = player_transform.translation.truncate();

            gizmos.line_2d(player_position, cursor_position, Color::YELLOW);
        }
    }
}
