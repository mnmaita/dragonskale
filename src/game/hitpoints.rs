use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug)]
pub struct Hitpoints {
    max: i16,
    current: i16,
}

impl Default for Hitpoints {
    fn default() -> Self {
        Self { max: 1, current: 1 }
    }
}

impl Hitpoints {
    pub fn new(max: i16) -> Self {
        if max <= 0 {
            panic!("Maximum Hitpoints cannot be zero or less.");
        }
        Self { max, current: max }
    }

    pub fn new_with_current(max: i16, current: impl Into<Option<i16>>) -> Self {
        if max <= 0 {
            panic!("Maximum Hitpoints cannot be zero or less.");
        }
        let current = current.into().unwrap_or(max).min(max);
        Self { max, current }
    }

    pub fn max(&self) -> i16 {
        self.max
    }

    pub fn current(&self) -> i16 {
        self.current
    }

    pub fn current_percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    pub fn set_max(&mut self, new_max: i16) {
        if new_max > 0 {
            let new_current = (new_max as f32 * self.current_percentage()) as i16;
            self.current = new_current.max(1);
            self.max = new_max;
        }
    }

    pub fn set_current(&mut self, new_current: i16) {
        self.current = new_current.min(self.max);
    }

    pub fn add_max(&mut self, hp: i16) {
        let new_max = self.max + hp;
        if new_max > 0 {
            let new_current = (new_max as f32 * self.current_percentage()) as i16;
            self.current = new_current.max(1);
            self.max = new_max;
        }
    }

    pub fn add(&mut self, hp: i16) {
        self.current = (self.current + hp).min(self.max);
    }

    pub fn subtract(&mut self, hp: i16) {
        self.current = (self.current - hp).max(0);
    }
}
