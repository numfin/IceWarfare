use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MouseAction {
    SetTarget,
}

pub fn system_listen_click_events(mut commands: Commands) {
    let input_map = InputMap::new([
        (MouseAction::SetTarget, InputKind::Mouse(MouseButton::Left)),
        (MouseAction::SetTarget, InputKind::Mouse(MouseButton::Right)),
    ]);

    commands.spawn(InputManagerBundle::with_map(input_map));
}
