use bevy::prelude::*;

use crate::controls::{IsControllable, PlayerAction};
use crate::movement::MoveTarget;

pub fn system(
    mut commands: Commands,
    players: Query<Entity, With<IsControllable>>,
    mut ev_reader: MessageReader<PlayerAction>,
) {
    for ev in ev_reader.read() {
        if let PlayerAction::ClickOnGround(target) = ev {
            for player in &players {
                commands.entity(player).insert(MoveTarget(*target));
            }
        }
    }
}
