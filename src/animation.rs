use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }

    pub fn first(&self) -> usize {
        self.first
    }

    pub fn last(&self) -> usize {
        self.last
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn from_seconds(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Repeating))
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            if let Some(ref mut texture_atlas) = sprite.texture_atlas {
                texture_atlas.index = if texture_atlas.index + 1 > indices.last {
                    indices.first
                } else {
                    texture_atlas.index + 1
                };
            }
        }
    }
}
