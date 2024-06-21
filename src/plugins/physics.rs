pub mod collision;

use bevy::prelude::*;
use bevy_xpbd_3d::{prelude::*, PhysicsSchedule, PhysicsStepSet, SubstepSet};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<collision::EventMovingHitMoving>()
            .add_event::<collision::EventHitWall>()
            .insert_resource(collision::AlreadyRunFlags::default())
            .add_systems(
                PhysicsSchedule,
                collision::system_reset_already_run.before(PhysicsStepSet::Substeps),
            )
            .add_systems(
                PostProcessCollisions,
                (
                    collision::system_handle_moving_hits,
                    collision::system_handle_wall_hits,
                )
                    .chain()
                    .in_set(SubstepSet::PostProcessCollisions),
            );
    }
}
