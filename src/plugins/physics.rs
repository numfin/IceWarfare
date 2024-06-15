pub mod already_run;
pub mod crash_into_smth;
pub mod custom_collision_report;

use already_run::AlreadyRun;
use bevy::prelude::*;
use bevy_xpbd_3d::{PhysicsSchedule, PhysicsStepSet, PostProcessCollisions, SubstepSet};
use crash_into_smth::CrashEvent;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CrashEvent>()
            .insert_resource(AlreadyRun(true))
            .add_systems(
                PhysicsSchedule,
                already_run::reset_already_run.before(PhysicsStepSet::Substeps),
            )
            .add_systems(
                PostProcessCollisions,
                crash_into_smth::crash_into_smth.in_set(SubstepSet::PostProcessCollisions),
            );
        // app.add_systems(PhysicsSchedule::, systems)
        // app.add_event::<custom_collision_report::TwoBodyImpact>()
        //     .add_systems(Update, custom_collision_report::report_contacts);
    }
}
