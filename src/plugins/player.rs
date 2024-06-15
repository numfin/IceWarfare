mod camera_follow_player;
mod move_player;
mod react_to_crash;
mod set_player_target;
mod spawn_player;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::plugins::sleeping::mark_sleeping_bodies;
use bevy_xpbd_3d::prelude::contact_reporting::report_contacts;
use bevy_xpbd_3d::{
    PhysicsSchedule, PhysicsSet, PhysicsStepSet, PostProcessCollisions, SubstepSchedule, SubstepSet,
};
use camera_follow_player::camera_follow_player;
use leafwing_input_manager::Actionlike;
use spawn_player::spawn_player;

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
    SetTarget,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControlPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    set_player_target::set_player_target,
                    react_to_crash::react_to_crash,
                ),
            )
            .add_systems(
                PhysicsSchedule,
                move_player::move_player.before(PhysicsStepSet::BroadPhase),
            )
            .add_systems(
                PostUpdate,
                camera_follow_player
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
