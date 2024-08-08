use bevy::prelude::*;
use tiny_bail::c;

use crate::health::{Health, HealthReference};
use crate::movement::MoveTarget;
use crate::physics::sys_handle_wall_hit::EventHitWall;

pub fn system(
    mut commands: Commands,
    players: Query<&HealthReference>,
    mut healths: Query<&mut Health>,
    mut ev_wall_hits: EventReader<EventHitWall>,
) {
    for ev in ev_wall_hits.read() {
        commands.entity(ev.entity).remove::<MoveTarget>();
        let health_ref = c!(players.get(ev.entity));
        if let Ok(mut health) = healths.get_mut(health_ref.0) {
            health.dmg(ev.velocity.length() * 10.0)
        }
    }
}
