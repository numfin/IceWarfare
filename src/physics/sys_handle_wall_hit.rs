use avian3d::prelude::*;
use bevy::prelude::*;

use crate::earth::spawn::Wall;

use super::sys_already_run::AlreadyRunFlags;

#[derive(Event)]
pub struct EventHitWall {
    pub entity: Entity,
    pub velocity: LinearVelocity,
}

pub fn observer(
    collisions: Res<Collisions>,
    mut already_run: ResMut<AlreadyRunFlags>,
    mut ev_crash: EventWriter<EventHitWall>,
    bodies: Query<&LinearVelocity>,
    walls: Query<&Wall>,
) {
    if already_run.handle_wall_hit {
        return;
    };

    for ((a, b), contacts) in collisions.get_internal() {
        let is_first_occurence = contacts.during_current_frame && !contacts.during_previous_frame;
        if !is_first_occurence {
            continue;
        }
        let (velocity, entity) =
            match (bodies.get(*a), walls.get(*b), walls.get(*a), bodies.get(*b)) {
                (Ok(a_body), Ok(_), ..) => (*a_body, *a),
                (.., Ok(_), Ok(b_body)) => (*b_body, *b),
                _ => continue,
            };
        already_run.handle_wall_hit = true;

        ev_crash.send(EventHitWall { entity, velocity });
    }
}
