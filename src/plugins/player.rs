mod camera;
mod components;
mod crash;
mod movement;
mod spawn;
mod target;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::{PhysicsSchedule, PhysicsSet, PhysicsStepSet};
use camera::system_camera_above_player;
use components::PlayerAction;
use spawn::spawn_player;

use super::controls::ControlPlugin;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControlPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, target::system_set_player_target)
            .add_systems(Update, crash::system_collide_with_moving_object)
            .add_systems(Update, movement::system_accelerate_over_time)
            .add_systems(Update, movement::system_stop_moving_when_reached_target)
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
