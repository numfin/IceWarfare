mod sys_already_run;
pub mod sys_handle_moving_hit_moving;
pub mod sys_handle_wall_hit;

use avian3d::prelude::{PhysicsSchedule, PhysicsStepSet, PostProcessCollisions};
use bevy::prelude::*;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<sys_handle_moving_hit_moving::EventMovingHitMoving>()
            .add_event::<sys_handle_wall_hit::EventHitWall>()
            .insert_resource(sys_already_run::AlreadyRunFlags::default())
            .add_systems(
                PhysicsSchedule,
                sys_already_run::system.before(PhysicsStepSet::First),
            )
            // .observe(sys_handle_moving_hit_moving::observer)
            .observe(sys_handle_wall_hit::observer);
    }
}
