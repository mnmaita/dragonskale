use std::f32::consts::FRAC_PI_2;

use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};

use crate::{game::Player, game::SpawnFireBreathEvent, playing, AppState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                clear_input.run_if(state_changed::<AppState>()),
                mouse_input.run_if(playing()),
            )
                .chain(),
        );
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
    mut spawn_fire_breath_event_writer: EventWriter<SpawnFireBreathEvent>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if mouse_input.pressed(MouseButton::Right) {
        if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
            let mut player_transform = query.single_mut();
            let player_position = player_transform.translation.truncate();
            let cursor_to_player_vector = cursor_position - player_position;
            let cursor_distance_to_player = cursor_position.distance(player_position);
            let velocity_rate = cursor_distance_to_player.min(300.) / 300.;

            if cursor_to_player_vector != Vec2::ZERO {
                let direction = cursor_to_player_vector.normalize();

                player_transform.translation.x += direction.x * 15. * velocity_rate;
                player_transform.translation.y += direction.y * 15. * velocity_rate;

                if direction != Vec2::ZERO {
                    let angle = (direction).angle_between(Vec2::X);

                    if angle.is_finite() {
                        // FIXME: Rotate the image sprite to always face right?
                        // FRAC_PI_2 is subtracted to offset the 90 degree rotation from the X axis the sprite has.
                        player_transform.rotation = Quat::from_rotation_z(-angle - FRAC_PI_2);
                    }
                }
            }
        }
    }

    if mouse_input.pressed(MouseButton::Left) {
        let player_transform = query.single();
        let player_direction = player_transform.rotation.mul_vec3(Vec3::Y).truncate();
        // TODO: replace constant with sprite dimensions
        let fire_position = player_transform.translation.truncate() + player_direction * 90.;

        spawn_fire_breath_event_writer.send(SpawnFireBreathEvent::new(1000, fire_position));
    }
}

fn clear_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut mouse_input: ResMut<Input<MouseButton>>,
) {
    keyboard_input.reset_all();
    mouse_input.reset_all()
}
