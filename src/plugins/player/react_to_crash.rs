use bevy::prelude::*;

use crate::plugins::health::{Health, HealthRelation};
use crate::plugins::physics::crash_into_smth::CrashEvent;

pub fn react_to_crash(
    mut ev_crash: EventReader<CrashEvent>,
    players: Query<&HealthRelation>,
    mut healths: Query<&mut Health>,
) {
    for CrashEvent { a, b } in ev_crash.read() {
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
