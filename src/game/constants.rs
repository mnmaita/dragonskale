use bevy::prelude::Vec2;

pub const TILE_SIZE: Vec2 = Vec2::new(32., 32.);
pub const GRID_SIZE: Vec2 = Vec2::new(100., 100.);
pub const HALF_TILE_SIZE: Vec2 = Vec2::new(TILE_SIZE.x * 0.5, TILE_SIZE.y * 0.5);
pub const HALF_GRID_SIZE: Vec2 = Vec2::new(GRID_SIZE.x * 0.5, GRID_SIZE.y * 0.5);
