use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug)]
pub struct ResourcePool<T> {
    max: i16,
    current: i16,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for ResourcePool<T> {
    fn default() -> Self {
        Self {
            max: 1,
            current: 1,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> ResourcePool<T> {
    pub fn new(max: i16) -> Self {
        if max <= 0 {
            panic!("Maximum ResourcePool cannot be zero or less.");
        }
        Self {
            max,
            current: max,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn new_with_current(max: i16, current: impl Into<Option<i16>>) -> Self {
        if max <= 0 {
            panic!("Maximum ResourcePool cannot be zero or less.");
        }
        let current = current.into().unwrap_or(max).min(max);
        Self {
            max,
            current,
            _marker: std::marker::PhantomData,
        }
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

    pub fn add_max(&mut self, value: i16) {
        let new_max = self.max + value;
        if new_max > 0 {
            let new_current = (new_max as f32 * self.current_percentage()) as i16;
            self.current = new_current.max(1);
            self.max = new_max;
        }
    }

    pub fn add(&mut self, value: i16) {
        self.current = (self.current + value).min(self.max);
    }

    pub fn subtract(&mut self, value: i16) {
        self.current = (self.current - value).max(0);
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0
    }
}
