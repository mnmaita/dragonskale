use bevy::{prelude::*, utils::HashMap};
use noise::{NoiseFn, Perlin};
use rand::random;

use crate::AppState;

pub const TILE_SIZE: Vec2 = Vec2::new(32., 32.);
pub const GRID_SIZE: Vec2 = Vec2::new(100., 100.);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), generate_level);
    }
}

pub fn generate_level(mut commands: Commands) {
    const MAP_OFFSET_X: f64 = 0.;
    const MAP_OFFSET_Y: f64 = 0.;
    const MAP_SCALE: f64 = 20.;

    let color_map: HashMap<u8, Color> = HashMap::from_iter([
        (0, Color::BLUE),
        (1, Color::BEIGE),
        (2, Color::DARK_GREEN),
        (3, Color::GRAY),
        (4, Color::DARK_GRAY),
    ]);
    let seed = random();
    let perlin = Perlin::new(seed);
    let color_map_len_f64 = color_map.len() as f64;

    for y in 0..GRID_SIZE.y as i32 {
        for x in 0..GRID_SIZE.x as i32 {
            let point = [
                (x as f64 - MAP_OFFSET_X) / MAP_SCALE,
                (y as f64 - MAP_OFFSET_Y) / MAP_SCALE,
            ];
            let noise_value = perlin.get(point).clamp(0., 1.);
            let scaled_noise_value =
                (noise_value * color_map_len_f64).clamp(0., color_map_len_f64 - 1.);
            let int_noise_value = scaled_noise_value.floor() as u8;
            let color = color_map
                .get(&int_noise_value)
                .unwrap_or(&Color::BLACK)
                .to_owned();
            let custom_size = Some(TILE_SIZE);
            let position =
                Vec2::new(x as f32 - GRID_SIZE.x / 2., y as f32 - GRID_SIZE.y / 2.) * TILE_SIZE;
            let translation = position.extend(0.0);
            let transform = Transform::from_translation(translation);

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size,
                    ..default()
                },
                transform,
                ..default()
            });
        }
    }
}

#[derive(Component, PartialEq, Eq, Hash)]
pub enum Tile {
    Water,
    Sand,
    Grass,
    Hills,
    Mountains,
}
