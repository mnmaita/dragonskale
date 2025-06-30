use std::{f32::consts::FRAC_PI_2, time::Duration};

use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};
use bevy_enhanced_input::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    animation::AnimationTimer, camera::MainCamera, game::Player, physics::Speed, playing, AppState,
};
use actions::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_enhanced_input::EnhancedInputPlugin);

        app.add_systems(
            PreUpdate,
            (
                clear_input.run_if(state_changed::<AppState>),
                player_movement.run_if(playing()),
            )
                .chain(),
        );

        app.add_input_context::<DefaultInputContext>();

        app.add_observer(bind_actions);
    }
}

#[derive(InputContext)]
pub struct DefaultInputContext;

pub mod actions {
    use super::InputAction;

    #[derive(Debug, InputAction)]
    #[input_action(output = bool)]
    pub struct FireBreath;
}

#[derive(SystemParam)]
pub struct CursorWorldPositionChecker<'w> {
    window: Single<'w, &'static Window, With<PrimaryWindow>>,
    main_camera: Single<'w, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
}

impl CursorWorldPositionChecker<'_> {
    pub fn cursor_world_position(&self) -> Option<Vec2> {
        self.window.cursor_position().and_then(|cursor_position| {
            let (camera, camera_transform) = *self.main_camera;
            camera
                .viewport_to_world_2d(camera_transform, cursor_position)
                .ok()
        })
    }
}

fn player_movement(
    player: Single<(&mut Transform, &Speed, &mut AnimationTimer, &Collider), With<Player>>,
    cursor_world_position_checker: CursorWorldPositionChecker,
) {
    if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
        let (mut player_transform, player_speed, mut player_animation_timer, player_collider) =
            player.into_inner();
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
                let angle = direction.angle_to(Vec2::X);

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
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut mouse_input: ResMut<ButtonInput<MouseButton>>,
) {
    keyboard_input.reset_all();
    mouse_input.reset_all()
}

fn bind_actions(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    let mut actions = Actions::<DefaultInputContext>::default();

    actions
        .bind::<FireBreath>()
        .to(MouseButton::Left)
        .with_conditions(Hold::new(0.5));

    commands.entity(trigger.target()).insert(actions);
}
