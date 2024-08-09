use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Resource)]
pub struct AlreadyRun<T: 'static> {
    flag: bool,
    _f: PhantomData<T>,
}
impl<T: 'static> AlreadyRun<T> {
    pub fn is_triggered(&self) -> bool {
        self.flag
    }
    pub fn reset(&mut self) {
        self.flag = false;
    }
    pub fn trigger(&mut self) {
        self.flag = true;
    }
}
impl<T> Default for AlreadyRun<T> {
    fn default() -> Self {
        Self {
            flag: false,
            _f: PhantomData,
        }
    }
}

pub fn system<T: Send + Sync>(mut r: ResMut<AlreadyRun<T>>) {
    r.reset();
}
