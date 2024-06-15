use bevy::prelude::*;

#[derive(Resource)]
pub struct AlreadyRun(pub bool);

pub fn reset_already_run(mut r: ResMut<AlreadyRun>) {
    r.0 = false;
}
