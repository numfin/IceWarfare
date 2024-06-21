use bevy::prelude::*;

use crate::plugins::health::{Health, HealthRelation};
use crate::plugins::physics::collision::{EventHitWall, EventMovingHitMoving};

pub fn system_collide_with_moving_object(
    mut ev_crash: EventReader<EventMovingHitMoving>,
    players: Query<&HealthRelation>,
    mut healths: Query<&mut Health>,
) {
    for EventMovingHitMoving { a, b } in ev_crash.read() {
        let impact_force = a.1.distance(b.1 .0);

        let Ok(a_health) = players.get(a.0) else {
            continue;
        };
        let Ok(b_health) = players.get(b.0) else {
            continue;
        };

        if let Ok(mut a_health) = healths.get_mut(a_health.health) {
            a_health.dmg(impact_force * b.1.length());
        }
        if let Ok(mut b_health) = healths.get_mut(b_health.health) {
            b_health.dmg(impact_force * a.1.length());
        }
    }
}

pub fn system_collide_with_wall(
    players: Query<&HealthRelation>,
    mut healths: Query<&mut Health>,
    mut ev_wall_hits: EventReader<EventHitWall>,
) {
    for ev in ev_wall_hits.read() {
        if let Ok(mut health) = players
            .get(ev.entity)
            .and_then(|p| healths.get_mut(p.health))
        {
            health.dmg(ev.velocity.length() * 10.0)
        }
    }
}
