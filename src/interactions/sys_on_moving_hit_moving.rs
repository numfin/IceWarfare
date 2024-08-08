use bevy::prelude::*;
use tiny_bail::cq;

use crate::health::{Health, HealthReference};
use crate::physics::sys_handle_moving_hit_moving::EventMovingHitMoving;

pub fn observer(
    mut ev_crash: EventReader<EventMovingHitMoving>,
    players: Query<&HealthReference>,
    mut healths: Query<&mut Health>,
) {
    for EventMovingHitMoving { a, b } in ev_crash.read() {
        let impact_force = a.1.distance(b.1 .0);

        let a_health = cq!(players.get(a.0));
        let b_health = cq!(players.get(b.0));

        if let Ok(mut a_health) = healths.get_mut(a_health.0) {
            a_health.dmg(impact_force * b.1.length());
        }
        if let Ok(mut b_health) = healths.get_mut(b_health.0) {
            b_health.dmg(impact_force * a.1.length());
        }
    }
}
