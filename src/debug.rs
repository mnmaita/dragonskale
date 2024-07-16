use bevy::{
    color::palettes::css::{FUCHSIA, RED, YELLOW},
    prelude::*,
};

use crate::{
    camera::MainCamera,
    game::{Player, GRID_SIZE, HALF_GRID_SIZE, HALF_LEVEL_SIZE, HALF_TILE_SIZE, TILE_SIZE},
    input::CursorWorldPositionChecker,
    playing,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (draw_grid, draw_camera_constraints, draw_mouse_direction).run_if(playing()),
        );
    }
}

fn draw_grid(mut gizmos: Gizmos) {
    gizmos.line_2d(
        Vec2::new(-HALF_TILE_SIZE.x, HALF_LEVEL_SIZE.y),
        Vec2::new(-HALF_TILE_SIZE.x, -HALF_LEVEL_SIZE.y - HALF_TILE_SIZE.y),
        FUCHSIA,
    );
    gizmos.line_2d(
        Vec2::new(HALF_LEVEL_SIZE.x, -HALF_TILE_SIZE.y),
        Vec2::new(-HALF_LEVEL_SIZE.x - HALF_TILE_SIZE.x, -HALF_TILE_SIZE.y),
        FUCHSIA,
    );
    for x in -HALF_GRID_SIZE.x as isize..HALF_GRID_SIZE.x as isize {
        let x: f32 = x as f32;
        gizmos.line_2d(
            Vec2::new(
                (x * TILE_SIZE.x) + HALF_TILE_SIZE.x * x.signum(),
                HALF_LEVEL_SIZE.y,
            ),
            Vec2::new(
                (x * TILE_SIZE.x) + HALF_TILE_SIZE.x * x.signum(),
                -HALF_LEVEL_SIZE.y - HALF_TILE_SIZE.y,
            ),
            FUCHSIA,
        );
    }
    for y in -HALF_GRID_SIZE.y as isize..HALF_GRID_SIZE.y as isize {
        let y: f32 = y as f32;
        gizmos.line_2d(
            Vec2::new(
                HALF_LEVEL_SIZE.x,
                (y * TILE_SIZE.y) + HALF_TILE_SIZE.y * y.signum(),
            ),
            Vec2::new(
                -HALF_LEVEL_SIZE.x - HALF_TILE_SIZE.x,
                (y * TILE_SIZE.y) + HALF_TILE_SIZE.y * y.signum(),
            ),
            FUCHSIA,
        );
    }
}

fn draw_camera_constraints(
    camera_query: Query<(&Camera, &Transform), (With<Camera2d>, With<MainCamera>)>,
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

        gizmos.rect_2d(camera_boundary.center(), 0., camera_boundary.size(), RED);
        gizmos.circle_2d(camera_transform.translation.truncate(), 10., RED);
    }
}

fn draw_mouse_direction(
    mouse_input: ResMut<ButtonInput<MouseButton>>,
    cursor_world_position_checker: CursorWorldPositionChecker,
    query: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
) {
    if mouse_input.pressed(MouseButton::Right) {
        if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
            let player_transform = query.single();
            let player_position = player_transform.translation.truncate();

            gizmos.line_2d(player_position, cursor_position, YELLOW);
        }
    }
}
