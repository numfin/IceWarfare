use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AlreadyRunFlags {
    pub handle_moving_hit: bool,
    pub handle_wall_hit: bool,
}
impl AlreadyRunFlags {
    pub fn reset(&mut self) {
        self.handle_moving_hit = false;
        self.handle_wall_hit = false;
    }
}

pub fn system(mut r: ResMut<AlreadyRunFlags>) {
    r.reset();
}
