use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioChannel, AudioControl};

use crate::{
    audio::DragonBreathChannel,
    camera::MainCamera,
    game::{Fire, Player, PlayerMovementEvent, ResourcePool, SpawnFireBreathEvent},
    playing, AppState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                clear_input.run_if(state_changed::<AppState>),
                mouse_input.run_if(playing()),
            )
                .chain(),
        );
    }
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

fn mouse_input(
    mut spawn_fire_breath_event_writer: EventWriter<SpawnFireBreathEvent>,
    mut player_movement_event_writer: EventWriter<PlayerMovementEvent>,
    cursor_world_position_checker: CursorWorldPositionChecker,
    player: Single<(&Transform, &ResourcePool<Fire>), With<Player>>,
    asset_server: Res<AssetServer>,
    mouse_input: ResMut<ButtonInput<MouseButton>>,
    dragon_breath_audio_channel: Res<AudioChannel<DragonBreathChannel>>,
    audio: Res<Audio>,
) {
    if let Some(cursor_position) = cursor_world_position_checker.cursor_world_position() {
        player_movement_event_writer.write(PlayerMovementEvent::accelerate(cursor_position));
    }

    let (player_transform, fire_breath_resource_pool) = player.into_inner();

    if mouse_input.just_pressed(MouseButton::Left) {
        dragon_breath_audio_channel.play(
            asset_server
                .get_handle("sfx/breathstart.ogg")
                .unwrap_or_default(),
        );
        dragon_breath_audio_channel
            .play(
                asset_server
                    .get_handle("sfx/breathloop.ogg")
                    .unwrap_or_default(),
            )
            .looped();
    } else if mouse_input.just_released(MouseButton::Left) {
        if !fire_breath_resource_pool.is_empty() {
            audio.play(
                asset_server
                    .get_handle("sfx/breathend.ogg")
                    .unwrap_or_default(),
            );
        }
        dragon_breath_audio_channel.stop();
    }

    if mouse_input.pressed(MouseButton::Left) {
        let player_direction = player_transform.rotation.mul_vec3(Vec3::Y).truncate();
        // TODO: replace constant with sprite dimensions
        let fire_position = player_transform.translation.truncate() + player_direction * 90.;

        if !fire_breath_resource_pool.is_empty() {
            spawn_fire_breath_event_writer.write(SpawnFireBreathEvent::new(1, fire_position));
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
