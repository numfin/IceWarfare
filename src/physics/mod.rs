mod sys_already_run;
// pub mod sys_handle_moving_hit_moving;
pub mod sys_handle_wall_hit;

use bevy::prelude::*;
// use sys_handle_moving_hit_moving::EventMovingHitMoving;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(sys_handle_wall_hit::system);
        // .init_resource::<AlreadyRun<EventMovingHitMoving>>()
        // .init_resource::<AlreadyRun<EventHitWall>>()
        // .add_systems(
        //     PhysicsSchedule,
        //     (
        //         sys_already_run::system::<EventHitWall>,
        //         // sys_already_run::system::<EventMovingHitMoving>,
        //     )
        //         .before(PhysicsStepSystems::First),
        // );
        // .add_systems(
        //     PostProcessCollisions,
        //     (
        //         sys_handle_wall_hit::system,
        //         sys_handle_moving_hit_moving::system,
        //     )
        //         .before(PhysicsStepSet::Solver),
        // );
    }
}
