mod sys_add_mouse_listener;
mod sys_handle_click_on_ground;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use sys_add_mouse_listener::ActionMouse;

#[derive(Debug, Event)]
pub enum EventPlayerAction {
    ClickOnGround(Vec2),
}

#[derive(Component)]
pub struct IsControllable;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventPlayerAction>()
            .add_plugins(InputManagerPlugin::<ActionMouse>::default())
            .add_systems(Startup, sys_add_mouse_listener::system)
            .add_systems(Update, sys_handle_click_on_ground::system);
    }
}
