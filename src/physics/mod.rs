mod sys_already_run;
pub mod sys_handle_moving_hit_moving;
pub mod sys_handle_wall_hit;

use avian3d::prelude::*;
use bevy::prelude::*;
use sys_already_run::AlreadyRun;
use sys_handle_moving_hit_moving::EventMovingHitMoving;
use sys_handle_wall_hit::EventHitWall;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .add_event::<EventMovingHitMoving>()
            .init_resource::<AlreadyRun<EventMovingHitMoving>>()
            .add_event::<EventHitWall>()
            .init_resource::<AlreadyRun<EventHitWall>>()
            .add_systems(
                PhysicsSchedule,
                (
                    sys_already_run::system::<EventHitWall>,
                    sys_already_run::system::<EventMovingHitMoving>,
                )
                    .before(PhysicsStepSet::First),
            )
            .add_systems(
                PostProcessCollisions,
                (
                    sys_handle_wall_hit::system,
                    sys_handle_moving_hit_moving::system,
                )
                    .before(PhysicsStepSet::Solver),
            );
    }
}
