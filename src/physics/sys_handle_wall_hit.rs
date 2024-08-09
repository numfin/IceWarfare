use avian3d::prelude::*;
use bevy::prelude::*;

use crate::earth::spawn::Wall;

use super::sys_already_run::AlreadyRun;

#[derive(Event)]
pub struct EventHitWall {
    pub body_id: Entity,
    pub wall_id: Entity,
    pub body_velocity: LinearVelocity,
}

pub fn system(
    collisions: Res<Collisions>,
    mut already_run: ResMut<AlreadyRun<EventHitWall>>,
    mut ev_hit_wall: EventWriter<EventHitWall>,
    bodies: Query<&LinearVelocity>,
    walls: Query<&Wall>,
) {
    if already_run.is_triggered() {
        return;
    };

    for ((a, b), contacts) in collisions.get_internal() {
        let is_first_occurence = contacts.during_current_frame && !contacts.during_previous_frame;
        if !is_first_occurence {
            continue;
        }

        let (body_velocity, body_id, wall_id) =
            match (bodies.get(*a), walls.get(*b), walls.get(*a), bodies.get(*b)) {
                (Ok(a_body), Ok(_b_wall), ..) => (*a_body, *a, *b),
                (.., Ok(_a_wall), Ok(b_body)) => (*b_body, *b, *a),
                _ => continue,
            };

        already_run.trigger();

        ev_hit_wall.send(EventHitWall {
            body_id,
            wall_id,
            body_velocity,
        });
    }
}
