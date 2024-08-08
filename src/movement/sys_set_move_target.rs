use bevy::prelude::*;

use crate::controls::{EventPlayerAction, IsControllable};
use crate::movement::MoveTarget;

pub fn system(
    mut commands: Commands,
    players: Query<Entity, With<IsControllable>>,
    mut ev_reader: EventReader<EventPlayerAction>,
) {
    for ev in ev_reader.read() {
        if let EventPlayerAction::ClickOnGround(target) = ev {
            for player in &players {
                commands.entity(player).insert(MoveTarget(*target));
            }
        }
    }
}
