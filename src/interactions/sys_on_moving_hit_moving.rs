use bevy::prelude::*;
use tiny_bail::cq;

use crate::health::Health;
use crate::physics::sys_handle_moving_hit_moving::EventMovingHitMoving;

pub fn system(mut ev_crash: EventReader<EventMovingHitMoving>, mut healths: Query<&mut Health>) {
    for EventMovingHitMoving { a, b } in ev_crash.read() {
        let impact_force = a.1.distance(b.1 .0);

        let mut a_health = cq!(healths.get_mut(a.0));
        a_health.dmg(impact_force * b.1.length());

        let mut b_health = cq!(healths.get_mut(b.0));
        b_health.dmg(impact_force * a.1.length());
    }
}
