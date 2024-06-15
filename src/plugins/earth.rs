pub mod spawn_earth;

use bevy::prelude::*;
use spawn_earth::spawn_earth;

pub struct EarthPlugin;
impl Plugin for EarthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_earth);
    }
}
