use bevy::prelude::*;

use crate::plugins::camera::cursor::EventClickGround;
use crate::plugins::target::Target;

use super::{Player, PlayerTargetRelation};

pub fn system_set_player_target(
    mut commands: Commands,
    mut player_targets: Query<&mut PlayerTargetRelation, With<Player>>,
    mut ev_reader: EventReader<EventClickGround>,
) {
    for EventClickGround(position) in ev_reader.read() {
        for mut player_target in &mut player_targets {
            if let Some(player_target_id) = player_target.target.take() {
                commands.entity(player_target_id).despawn();
            }
            let player_target_id = commands.spawn(Target::Earth(*position)).id();
            let _ = player_target.target.insert(player_target_id);
        }
    }
}
