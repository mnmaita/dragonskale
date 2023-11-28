use bevy::{
    audio::{Volume, VolumeLevel},
    ecs::system::SystemParam,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_rapier2d::prelude::*;
use noise::{NoiseFn, Perlin};
use pathfinding::prelude::Matrix;
use rand::{random, seq::SliceRandom};

use crate::{
    audio::{PlayMusicEvent, SoundEffect},
    camera::{YSorted, BACKGROUND_LAYER, GROUND_LAYER},
    entity_cleanup,
    game::{
        InGameEntity, BUILDING_GROUP, ENEMY_GROUP, FIRE_BREATH_GROUP, GRID_SIZE, HALF_GRID_SIZE,
        HALF_TILE_SIZE, TILE_SIZE,
    },
    AppState,
};

use super::{
    combat::{AttackDamage, AttackTimer, Range},
    resource_pool::{Health, ResourcePool},
    Enemy,
};

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            (generate_level_matrix, generate_tilemaps),
        );

        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_level_tiles, spawn_buildings, play_background_music).chain(),
        );

        app.add_systems(
            OnExit(AppState::InGame),
            entity_cleanup::<With<SoundEffect>>,
        );
    }
}

fn generate_tilemaps(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tileset_ground_texture = asset_server
        .get_handle("textures/tileset_ground.png")
        .unwrap_or_default();
    let tileset_objects_texture = asset_server
        .get_handle("textures/tileset_objects.png")
        .unwrap_or_default();
    let tileset_ground_texture_atlas =
        TextureAtlas::from_grid(tileset_ground_texture, TILE_SIZE, 16, 18, None, None);
    let tileset_objects_texture_atlas =
        TextureAtlas::from_grid(tileset_objects_texture, TILE_SIZE, 38, 14, None, None);

    commands.insert_resource(TilesetGroundTextureAtlasHandle(
        asset_server.add(tileset_ground_texture_atlas),
    ));
    commands.insert_resource(TilesetObjectsTextureAtlasHandle(
        asset_server.add(tileset_objects_texture_atlas),
    ));
}

fn generate_level_matrix(mut commands: Commands) {
    const MAP_OFFSET_X: f64 = 0.;
    const MAP_OFFSET_Y: f64 = 0.;
    const MAP_SCALE: f64 = 20.;

    let seed = random();
    let perlin = Perlin::new(seed);
    let tile_count = Tile::_LAST as u8;
    let mut level_matrix = Matrix::new(GRID_SIZE.y as usize, GRID_SIZE.x as usize, Tile::_LAST);

    for y in 0..GRID_SIZE.y as usize {
        for x in 0..GRID_SIZE.x as usize {
            let point = [
                (x as f64 - MAP_OFFSET_X) / MAP_SCALE,
                (y as f64 - MAP_OFFSET_Y) / MAP_SCALE,
            ];
            let noise_value = perlin.get(point).clamp(0., 1.);
            let scaled_noise_value =
                (noise_value * tile_count as f64).clamp(0., tile_count as f64 - 1.);
            let int_noise_value = scaled_noise_value.floor() as u8;

            if let Some(tile) = level_matrix.get_mut((x, y)) {
                *tile = int_noise_value.into();
            }
        }
    }

    commands.insert_resource(LevelMatrix(level_matrix));
}

fn spawn_level_tiles(
    mut commands: Commands,
    level_matrix: Res<LevelMatrix>,
    tileset_ground_texture_atlas_handle: Res<TilesetGroundTextureAtlasHandle>,
) {
    for ((x, y), tile) in level_matrix.0.items() {
        let tile = *tile;
        let position = translate_grid_position_to_world_space(&(x, y));
        let translation = position.extend(0.0);
        let transform = Transform::from_translation(translation);
        let mut tile_entity = commands.spawn(TileBundle {
            render_layers: RenderLayers::layer(BACKGROUND_LAYER),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(tile.into()),
                texture_atlas: tileset_ground_texture_atlas_handle.clone(),
                transform,
                ..default()
            },
            tile,
        });

        tile_entity.insert(InGameEntity);

        if y == 0 || x == 0 || y == GRID_SIZE.y as usize - 1 || x == GRID_SIZE.x as usize - 1 {
            tile_entity.insert(BorderTile);
        }
    }
}

fn spawn_buildings(mut commands: Commands, level_matrix: Res<LevelMatrix>) {
    const BUILDING_SPAWN_CHANCE: f32 = 0.01;

    let grass_tiles: Vec<Vec2> = level_matrix
        .items()
        .filter(|(_, tile)| **tile == Tile::Grass)
        .map(|(pos, _)| translate_grid_position_to_world_space(&pos))
        .collect();
    let total_buildings = (grass_tiles.len() as f32 * BUILDING_SPAWN_CHANCE).ceil() as usize;
    let mut rng = rand::thread_rng();
    let random_spawn_points = grass_tiles.choose_multiple(&mut rng, total_buildings);

    for position in random_spawn_points {
        let translation = position.extend(1.);
        let mut building_entity_commands = commands.spawn(BuildingBundle {
            active_collision_types: ActiveCollisionTypes::all(),
            attack_damage: AttackDamage(5),
            attack_timer: AttackTimer::new(6.),
            collider: Collider::ball(HALF_TILE_SIZE.x),
            collision_groups: CollisionGroups::new(BUILDING_GROUP, ENEMY_GROUP | FIRE_BREATH_GROUP),
            hitpoints: ResourcePool::<Health>::new(1000),
            marker: Enemy,
            range: Range(TILE_SIZE.x * 15.),
            render_layers: RenderLayers::layer(GROUND_LAYER),
            rigid_body: RigidBody::Fixed,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::DARK_GRAY,
                    custom_size: Some(TILE_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(translation),
                ..default()
            },
        });

        building_entity_commands.insert((InGameEntity, YSorted));
    }
}

fn play_background_music(mut play_music_event_writer: EventWriter<PlayMusicEvent>) {
    play_music_event_writer.send(PlayMusicEvent::new(
        "theme2.ogg",
        Some(PlaybackSettings {
            volume: Volume::Absolute(VolumeLevel::new(0.25)),
            ..default()
        }),
        None,
    ));
}

#[derive(Resource, Deref)]
pub struct TilesetGroundTextureAtlasHandle(Handle<TextureAtlas>);

#[derive(Resource, Deref)]
pub struct TilesetObjectsTextureAtlasHandle(Handle<TextureAtlas>);

#[derive(Resource, Deref)]
pub struct LevelMatrix(Matrix<Tile>);

#[derive(Bundle)]
pub struct BuildingBundle {
    pub active_collision_types: ActiveCollisionTypes,
    pub attack_damage: AttackDamage,
    pub attack_timer: AttackTimer,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub hitpoints: ResourcePool<Health>,
    pub marker: Enemy,
    pub range: Range,
    pub sprite: SpriteBundle,
    pub render_layers: RenderLayers,
    pub rigid_body: RigidBody,
}

#[derive(Component)]
pub struct BorderTile;

#[derive(Bundle)]
pub struct TileBundle {
    pub render_layers: RenderLayers,
    pub sprite: SpriteSheetBundle,
    pub tile: Tile,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Water,
    Sand,
    Grass,
    Hills,
    Mountains,
    _LAST,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        // For every new type added to the enum, a new match arm should be added here.
        match value {
            0 => Self::Water,
            1 => Self::Sand,
            2 => Self::Grass,
            3 => Self::Hills,
            4 => Self::Mountains,
            #[cfg(debug_assertions)]
            _ => panic!("From<u8> for Tile: Missing match arm!"),
            #[cfg(not(debug_assertions))]
            _ => Self::Water,
        }
    }
}

impl From<Tile> for Color {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Grass => Self::DARK_GREEN,
            Tile::Hills => Self::GRAY,
            Tile::Mountains => Self::DARK_GRAY,
            Tile::Water => Self::BLUE,
            Tile::Sand => Self::BEIGE,
            Tile::_LAST => Self::default(),
        }
    }
}

impl From<Tile> for usize {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Grass => 34,
            Tile::Hills => 242,
            Tile::Mountains => 242,
            Tile::Sand => 183,
            Tile::Water => {
                if random::<f32>() > 0.1 {
                    145
                } else {
                    let mut rng = rand::thread_rng();
                    *[146_usize, 147, 148].choose(&mut rng).unwrap()
                }
            }
            Tile::_LAST => 0,
            _ => 0,
        }
    }
}

#[derive(SystemParam)]
pub struct TileQuery<'w, 's> {
    tile_query: Query<'w, 's, (&'static Tile, &'static Transform)>,
}

impl<'w, 's> TileQuery<'w, 's> {
    pub fn get_from_position(&self, pos: Vec2) -> Option<&Tile> {
        self.tile_query
            .iter()
            .find(|(_, transform)| {
                let pos_transform = &Transform::from_translation(pos.extend(0.));
                translate_transform_to_grid_space(transform)
                    == translate_transform_to_grid_space(pos_transform)
            })
            .map(|(tile, _)| tile)
    }
}

pub fn translate_transform_to_grid_space(transform: &Transform) -> (usize, usize) {
    let half_columns = GRID_SIZE.x * 0.5;
    let half_rows = GRID_SIZE.y * 0.5;
    let x = ((transform.translation.x / TILE_SIZE.x) + half_columns).round();
    let y = ((transform.translation.y / TILE_SIZE.y) + half_rows).round();
    if x >= 0.0 && y >= 0.0 {
        (x as usize, y as usize)
    } else {
        (0, 0)
    }
}

pub fn translate_grid_position_to_world_space(pos: &(usize, usize)) -> Vec2 {
    let x = (pos.0 as f32 - HALF_GRID_SIZE.x) * TILE_SIZE.x;
    let y = (pos.1 as f32 - HALF_GRID_SIZE.y) * TILE_SIZE.y;
    Vec2::new(x, y)
}
