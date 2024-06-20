mod camera;
mod crash;
mod movement;
mod spawn;
mod target;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::{PhysicsSchedule, PhysicsSet, PhysicsStepSet};
use camera::system_camera_above_player;
use leafwing_input_manager::Actionlike;
use spawn::spawn_player;

use super::controls::ControlPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerTargetRelation {
    pub target: Option<Entity>,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {
    Run,
    Stop,
    RotateLeft,
    RotateRight,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControlPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, target::system_set_player_target)
            .add_systems(Update, crash::system_collide_with_moving_object)
            .add_systems(
                PhysicsSchedule,
                movement::system_move_towards_target.before(PhysicsStepSet::BroadPhase),
            )
            .add_systems(
                PostUpdate,
                system_camera_above_player
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
