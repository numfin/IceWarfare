use bevy::prelude::*;

use crate::plugins::camera::mouse::EventClickGround;

use super::components::{Player, PlayerTarget};

pub fn system_set_player_target(
    mut player_targets: Query<&mut PlayerTarget, With<Player>>,
    mut ev_reader: EventReader<EventClickGround>,
) {
    for EventClickGround(position) in ev_reader.read() {
        for mut player_target in &mut player_targets {
            player_target.0.replace(*position);
        }
    }
}
