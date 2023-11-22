use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug)]
pub struct ScoreSystem<T> {
    current: i32,
    multiplier: i16,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for ScoreSystem<T> {
    fn default() -> Self {
        Self {
            current: 1,
            multiplier: 1,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> ScoreSystem<T> {
    pub fn new(current: i32, multiplier: i16) -> Self {
        Self {
            current,
            multiplier,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn multiplier(&self) -> i16 {
        self.multiplier
    }

    pub fn set_current(&mut self, new_current: i32) {
        self.current = new_current
    }

    pub fn set_multiplier(&mut self, new_multiplier: i16) {
        self.multiplier = new_multiplier;
    }

    pub fn increase_multiplier_by_one(&mut self) {
        self.multiplier += 1;
    }

    pub fn reset_multiplier(&mut self) {
        self.multiplier = 1;
    }

    pub fn add_with_multiplier(&mut self, value: i32) {
        self.current += value * self.multiplier as i32;
    }

    pub fn subtract(&mut self, value: i32) {
        self.current = self.current - value;
    }
}

#[derive(Component)]
pub struct Score;
