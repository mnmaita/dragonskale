use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};

use crate::{game::Player, playing};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input.run_if(playing()));
    }
}

#[derive(Resource)]
struct CursorWorldPosition(Option<Vec2>);

#[derive(SystemParam)]
struct CursorWorldPositionChecker<'w, 's> {
    window_query: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<Camera2d>>,
}

impl CursorWorldPositionChecker<'_, '_> {
    pub fn cursor_world_position(&self) -> Option<Vec2> {
        let window = self.window_query.single();

        window.cursor_position().and_then(|cursor_position| {
            let (camera, camera_transform) = self.camera_query.single();
            camera.viewport_to_world_2d(camera_transform, cursor_position)
        })
    }
}

fn mouse_input(
    mouse_input: ResMut<Input<MouseButton>>,
    cursor_world_position_checker: CursorWorldPositionChecker,
    mut query: Query<&mut Transform, With<Player>>,
    mut gizmos: Gizmos,
) {
    if mouse_input.pressed(MouseButton::Right) {
        if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
            let mut player_transform = query.single_mut();
            let player_position = player_transform.translation.truncate();
            let cursor_distance_to_player = cursor_position.distance(player_position);
            let velocity_rate = cursor_distance_to_player.min(300.) / 300.;
            let direction = (cursor_position - player_position).normalize();

            player_transform.translation.x += direction.x * 15. * velocity_rate;
            player_transform.translation.y += direction.y * 15. * velocity_rate;

            #[cfg(debug_assertions)]
            gizmos.line_2d(player_position, cursor_position, Color::YELLOW);
        }
    }
}
