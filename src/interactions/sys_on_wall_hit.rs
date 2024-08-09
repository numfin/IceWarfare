use bevy::prelude::*;
use tiny_bail::c;

use crate::health::Health;
use crate::movement::MoveTarget;
use crate::physics::sys_handle_wall_hit::EventHitWall;

pub fn system(
    mut commands: Commands,
    mut players: Query<&mut Health>,
    mut ev_wall_hits: EventReader<EventHitWall>,
) {
    for ev in ev_wall_hits.read() {
        commands.entity(ev.body_id).remove::<MoveTarget>();
        let mut health = c!(players.get_mut(ev.body_id));
        health.dmg(ev.body_velocity.length() * 10.0)
    }
}
