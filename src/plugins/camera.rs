mod spawn_camera;

use bevy::prelude::*;
use spawn_camera::spawn_camera;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}
