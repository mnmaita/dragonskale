use bevy::{
    prelude::*,
    render::view::{Layer, RenderLayers},
};

use crate::{
    game::Player,
    game::{GRID_SIZE, HALF_TILE_SIZE, TILE_SIZE},
};

pub enum RenderLayer {
    Background = 0,
    Topography,
    Ground,
    Sky,
    Ui,
}

impl From<RenderLayer> for Layer {
    fn from(value: RenderLayer) -> Self {
        value as Layer
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);

        app.add_systems(
            PostUpdate,
            (
                update_camera.run_if(any_with_component::<Player>),
                constrain_camera_position_to_level.after(update_camera),
                y_sorting,
                inverse_y_sorting,
            ),
        );
    }
}

fn sub_layer_camera(layer: Layer) -> impl Bundle {
    (
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::None,
            order: layer as isize,
            ..default()
        },
        Msaa::Off,
        RenderLayers::layer(layer),
    )
}

#[derive(Component)]
#[require(
    Camera2d,
    Camera {
        order: RenderLayer::Background as isize,
        ..Default::default()
    },
    Msaa::Off,
    RenderLayers::layer(RenderLayer::Background.into())
)]
pub struct MainCamera;

#[derive(Component)]
pub struct YSorted;

#[derive(Component)]
pub struct YSortedInverse;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        children![
            sub_layer_camera(RenderLayer::Ground.into()),
            sub_layer_camera(RenderLayer::Topography.into()),
            sub_layer_camera(RenderLayer::Sky.into()),
            sub_layer_camera(RenderLayer::Ui.into()),
        ],
    ));
}

fn update_camera(
    main_camera: Single<&mut Transform, With<MainCamera>>,
    player_transform: Single<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera_transform = main_camera.into_inner();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn constrain_camera_position_to_level(
    main_camera: Single<(&Camera, &mut Transform), With<MainCamera>>,
) {
    let (camera, mut camera_transform) = main_camera.into_inner();

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

pub fn y_sorting(mut query: Query<&mut Transform, (Changed<Transform>, With<YSorted>)>) {
    for mut transform in &mut query {
        transform.translation.z = transform.translation.normalize().y;
    }
}

pub fn inverse_y_sorting(
    mut query: Query<&mut Transform, (Changed<Transform>, With<YSortedInverse>)>,
) {
    for mut transform in &mut query {
        transform.translation.z = -transform.translation.normalize().y;
    }
}
