use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use super::already_run::AlreadyRun;

#[derive(Event)]
pub struct CrashEvent {
    pub a: (Entity, LinearVelocity),
    pub b: (Entity, LinearVelocity),
}

pub fn crash_into_smth(
    collisions: Res<Collisions>,
    mut already_run: ResMut<AlreadyRun>,
    mut ev_crash: EventWriter<CrashEvent>,
    players: Query<&LinearVelocity>,
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

        let Ok(a_velocity) = players.get(*a) else {
            continue;
        };
        let Ok(b_velocity) = players.get(*b) else {
            continue;
        };
        ev_crash.send(CrashEvent {
            a: (*a, *a_velocity),
            b: (*b, *b_velocity),
        });

        // let impact_force = a_velocity.distance(b_velocity.0);

        // if let Ok(mut a_health) = healths.get_mut(a_health.health) {
        //     dbg!(impact_force * b_velocity.length());
        //     a_health.dmg(impact_force * b_velocity.length());
        // }
        // if let Ok(mut b_health) = healths.get_mut(b_health.health) {
        //     dbg!(impact_force * a_velocity.length());
        //     b_health.dmg(impact_force * a_velocity.length());
        // }
    }
}
