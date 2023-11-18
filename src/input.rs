use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};

use crate::{game::PlayerMovementEvent, playing};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, mouse_input.run_if(playing()));
    }
}

#[derive(Resource)]
struct CursorWorldPosition(Option<Vec2>);

#[derive(SystemParam)]
pub struct CursorWorldPositionChecker<'w, 's> {
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
    mut player_movement_event_writer: EventWriter<PlayerMovementEvent>,
) {
    if mouse_input.pressed(MouseButton::Right) {
        if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
            player_movement_event_writer.send(PlayerMovementEvent::accelerate(cursor_position));
        }
    } else if mouse_input.just_released(MouseButton::Right) {
        player_movement_event_writer.send(PlayerMovementEvent::brake());
    }
}
