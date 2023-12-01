use std::{f32::consts::FRAC_PI_2, time::Duration};

use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::Collider;

use crate::{
    animation::AnimationTimer, camera::MainCamera, game::Player, game::SpawnFireBreathEvent,
    physics::Speed, playing, AppState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                clear_input.run_if(state_changed::<AppState>()),
                (mouse_input, player_movement).run_if(playing()),
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
    camera_query: Query<
        'w,
        's,
        (&'static Camera, &'static GlobalTransform),
        (With<Camera2d>, With<MainCamera>),
    >,
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

#[derive(Component)]
struct FireBreathSfx;

fn mouse_input(
    mut commands: Commands,
    mut spawn_fire_breath_event_writer: EventWriter<SpawnFireBreathEvent>,
    query: Query<&Transform, With<Player>>,
    sfx_query: Query<Entity, With<FireBreathSfx>>,
    asset_server: Res<AssetServer>,
    mouse_input: ResMut<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        commands.spawn((
            AudioBundle {
                source: asset_server
                    .get_handle("sfx/breathstart.ogg")
                    .unwrap_or_default(),
                settings: PlaybackSettings::DESPAWN,
            },
            FireBreathSfx,
        ));
        commands.spawn((
            AudioBundle {
                source: asset_server
                    .get_handle("sfx/breathloop.ogg")
                    .unwrap_or_default(),
                settings: PlaybackSettings::LOOP,
            },
            FireBreathSfx,
        ));
    } else if mouse_input.just_released(MouseButton::Left) {
        commands.spawn((
            AudioBundle {
                source: asset_server
                    .get_handle("sfx/breathend.ogg")
                    .unwrap_or_default(),
                settings: PlaybackSettings::DESPAWN,
            },
            FireBreathSfx,
        ));
        for entity in &sfx_query {
            commands.entity(entity).despawn_recursive();
        }
    }

    if mouse_input.pressed(MouseButton::Left) {
        let player_transform = query.single();
        let player_direction = player_transform.rotation.mul_vec3(Vec3::Y).truncate();
        // TODO: replace constant with sprite dimensions
        let fire_position = player_transform.translation.truncate() + player_direction * 90.;

        spawn_fire_breath_event_writer.send(SpawnFireBreathEvent::new(1, fire_position));
    }
}

fn player_movement(
    mut query: Query<(&mut Transform, &Speed, &mut AnimationTimer, &Collider), With<Player>>,
    cursor_world_position_checker: CursorWorldPositionChecker,
) {
    if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
        let (mut player_transform, player_speed, mut player_animation_timer, player_collider) =
            query.single_mut();
        let player_position = player_transform.translation.truncate();
        let cursor_to_player_vector = cursor_position - player_position;

        if cursor_to_player_vector != Vec2::ZERO {
            let cursor_distance_to_player = cursor_position.distance(player_position);
            let velocity_rate = cursor_distance_to_player.min(300.) / 300.;
            let direction = cursor_to_player_vector.normalize();

            if cursor_distance_to_player > player_collider.as_cuboid().unwrap().half_extents().y {
                player_transform.translation.x += direction.x * player_speed.0 * velocity_rate;
                player_transform.translation.y += direction.y * player_speed.0 * velocity_rate;
                player_animation_timer.set_duration(Duration::from_secs_f32(
                    0.2 * player_speed.0 * 0.25 * velocity_rate,
                ));
            } else {
                player_animation_timer.set_duration(Duration::from_secs_f32(0.2));
            }

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

fn clear_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut mouse_input: ResMut<Input<MouseButton>>,
) {
    keyboard_input.reset_all();
    mouse_input.reset_all()
}
