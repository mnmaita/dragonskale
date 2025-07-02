use core::{marker::PhantomData, time::Duration};

use bevy::{prelude::*, time::Timer};

#[derive(Component, Resource, Default, Deref, DerefMut)]
pub struct GameTimer<T> {
    #[deref]
    timer: Timer,
    _marker: PhantomData<T>,
}

impl<T> GameTimer<T> {
    pub fn from_seconds(secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Repeating),
            _marker: PhantomData,
        }
    }

    pub fn from_seconds_once(secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Once),
            _marker: PhantomData,
        }
    }

    pub fn replace_seconds(&mut self, secs: f32) {
        self.timer.set_duration(Duration::from_secs_f32(secs));
    }
}
