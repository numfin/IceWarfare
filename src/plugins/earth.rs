pub mod spawn;

use bevy::prelude::*;

pub struct EarthPlugin;
impl Plugin for EarthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::system_spawn_bounds);
    }
}
