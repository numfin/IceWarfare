pub mod collision;

use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule, PhysicsStepSet, SubstepSet};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<collision::EventCollisionOfTwo>()
            .insert_resource(collision::AlreadyRun(true))
            .add_systems(
                PhysicsSchedule,
                collision::system_reset_already_run.before(PhysicsStepSet::Substeps),
            )
            .add_systems(
                PostProcessCollisions,
                collision::system_handle_moving_collisions
                    .in_set(SubstepSet::PostProcessCollisions),
            );
    }
}
