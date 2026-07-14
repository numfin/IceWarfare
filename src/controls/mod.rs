mod sys_add_mouse_listener;
mod sys_handle_click_on_ground;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use sys_add_mouse_listener::ActionMouse;

#[derive(Debug, Message)]
pub enum PlayerAction {
    ClickOnGround(Vec2),
}

#[derive(Component)]
pub struct IsControllable;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerAction>()
            .add_plugins(InputManagerPlugin::<ActionMouse>::default())
            .add_systems(Startup, sys_add_mouse_listener::system)
            .add_systems(Update, sys_handle_click_on_ground::system);
    }
}
