use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerTarget(pub Option<Vec3>);

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Ability1,
}
