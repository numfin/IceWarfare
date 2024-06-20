mod camera;
pub mod cursor;
pub mod mouse;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use mouse::MouseAction;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InputManagerPlugin::<MouseAction>::default())
            .add_event::<cursor::EventClickGround>()
            .add_systems(Startup, camera::system_spawn_camera)
            .add_systems(Startup, mouse::system_listen_click_events)
            .add_systems(Update, cursor::system_handle_click_on_ground);
    }
}
