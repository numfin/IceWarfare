use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::plugins::earth::spawn::Wall;

#[derive(Resource, Default)]
pub struct AlreadyRunFlags {
    pub handle_moving_hit: bool,
    pub handle_wall_hit: bool,
}
impl AlreadyRunFlags {
    pub fn reset(&mut self) {
        self.handle_moving_hit = false;
        self.handle_wall_hit = false;
    }
}

pub fn system_reset_already_run(mut r: ResMut<AlreadyRunFlags>) {
    r.reset();
}

#[derive(Event)]
pub struct EventMovingHitMoving {
    pub a: (Entity, LinearVelocity),
    pub b: (Entity, LinearVelocity),
}

pub fn system_handle_moving_hits(
    collisions: Res<Collisions>,
    mut already_run: ResMut<AlreadyRunFlags>,
    mut ev_crash: EventWriter<EventMovingHitMoving>,
    bodies: Query<&LinearVelocity>,
) {
    if already_run.handle_moving_hit {
        return;
    };

    for ((a, b), contacts) in collisions.get_internal() {
        let is_first_occurence = contacts.during_current_frame && !contacts.during_previous_frame;
        if !is_first_occurence {
            continue;
        }

        let (Ok(a_velocity), Ok(b_velocity)) = (bodies.get(*a), bodies.get(*b)) else {
            continue;
        };
        already_run.handle_moving_hit = true;

        ev_crash.send(EventMovingHitMoving {
            a: (*a, *a_velocity),
            b: (*b, *b_velocity),
        });
    }
}

#[derive(Event)]
pub struct EventHitWall {
    pub entity: Entity,
    pub velocity: LinearVelocity,
}
pub fn system_handle_wall_hits(
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
