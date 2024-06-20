pub mod mouse;
mod spawn;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use mouse::MouseAction;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InputManagerPlugin::<MouseAction>::default())
            .add_event::<mouse::EventClickGround>()
            .add_systems(Startup, spawn::system_spawn_camera)
            .add_systems(Startup, mouse::system_listen_click_events)
            .add_systems(Update, mouse::system_handle_click_on_ground);
    }
}
