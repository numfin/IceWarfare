use avian3d::prelude::*;
use bevy::prelude::*;
use tiny_bail::cq;

use super::sys_already_run::AlreadyRun;

#[derive(Event)]
pub struct EventMovingHitMoving {
    pub a: (Entity, LinearVelocity),
    pub b: (Entity, LinearVelocity),
}

pub fn system(
    collisions: Res<Collisions>,
    mut already_run: ResMut<AlreadyRun<EventMovingHitMoving>>,
    mut ev_crash: EventWriter<EventMovingHitMoving>,
    bodies: Query<&LinearVelocity>,
) {
    if already_run.is_triggered() {
        return;
    };

    for ((a, b), contacts) in collisions.get_internal() {
        let is_first_occurence = contacts.during_current_frame && !contacts.during_previous_frame;
        if !is_first_occurence {
            continue;
        }

        let (a_velocity, b_velocity) = (cq!(bodies.get(*a)), cq!(bodies.get(*b)));
        already_run.trigger();

        ev_crash.send(EventMovingHitMoving {
            a: (*a, *a_velocity),
            b: (*b, *b_velocity),
        });
    }
}
