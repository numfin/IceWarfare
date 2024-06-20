use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Resource)]
pub struct AlreadyRun(pub bool);

pub fn system_reset_already_run(mut r: ResMut<AlreadyRun>) {
    r.0 = false;
}

#[derive(Event)]
pub struct EventCollisionOfTwo {
    pub a: (Entity, LinearVelocity),
    pub b: (Entity, LinearVelocity),
}

pub fn system_handle_moving_collisions(
    collisions: Res<Collisions>,
    mut already_run: ResMut<AlreadyRun>,
    mut ev_crash: EventWriter<EventCollisionOfTwo>,
    bodies: Query<&LinearVelocity>,
) {
    if already_run.0 {
        return;
    };

    for ((a, b), contacts) in collisions.get_internal() {
        let is_first_occurence = contacts.during_current_frame && !contacts.during_previous_frame;
        if !is_first_occurence {
            continue;
        }
        already_run.0 = true;

        let (Ok(a_velocity), Ok(b_velocity)) = (bodies.get(*a), bodies.get(*b)) else {
            continue;
        };
        ev_crash.send(EventCollisionOfTwo {
            a: (*a, *a_velocity),
            b: (*b, *b_velocity),
        });
    }
}
