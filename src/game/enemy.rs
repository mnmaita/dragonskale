use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{physics::Speed, playing};

use super::{combat::Range, BorderTile, Hitpoints, Player, TILE_SIZE};

pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::new(3.));

        app.add_systems(FixedUpdate, (spawn_enemies, move_enemies).run_if(playing()));
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub marker: Enemy,
    pub hitpoints: Hitpoints,
    pub range: Range,
    pub speed: Speed,
    pub sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Resource, Deref, DerefMut)]
struct EnemySpawnTimer(Timer);

impl EnemySpawnTimer {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    tile_query: Query<&Transform, With<BorderTile>>,
) {
    if enemy_spawn_timer.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        if let Some(tile_transform) = tile_query.iter().choose(&mut rng) {
            let translation = tile_transform.translation.truncate().extend(1.);

            commands.spawn(EnemyBundle {
                hitpoints: Hitpoints::new(1),
                marker: Enemy,
                range: Range(TILE_SIZE.x * 3.),
                speed: Speed(2.),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(TILE_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(translation),
                    ..default()
                },
            });
        }
    }
}

fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &Speed, &Range), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    let player_position = player_transform.translation.truncate();

    for (mut enemy_transform, enemy_speed, enemy_range) in &mut enemy_query {
        let enemy_position = enemy_transform.translation.truncate();
        if enemy_position.distance(player_position) > enemy_range.0 {
            let enemy_direction = (player_position - enemy_position).normalize();
            enemy_transform.translation.x += enemy_direction.x * enemy_speed.0;
            enemy_transform.translation.y += enemy_direction.y * enemy_speed.0;
        }
    }
}
