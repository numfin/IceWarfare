use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ActionMouse {
    Click,
}

pub fn system(mut commands: Commands) {
    let input_map = InputMap::new([
        (ActionMouse::Click, InputKind::Mouse(MouseButton::Left)),
        // ,
    ]);

    commands.spawn(InputManagerBundle::with_map(input_map));
}
