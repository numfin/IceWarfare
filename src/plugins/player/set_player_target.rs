use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::plugins::cursor::GameCursor;
use crate::plugins::target::Target;

use super::{Player, PlayerAction, PlayerTargetRelation};

pub fn set_player_target(
    mut commands: Commands,
    mut player_actions: Query<
        (&ActionState<PlayerAction>, &mut PlayerTargetRelation),
        With<Player>,
    >,
    cursors: Query<&Transform, With<GameCursor>>,
) {
    for (_, mut player_target) in player_actions
        .iter_mut()
        .filter(|(action, ..)| action.just_pressed(&PlayerAction::SetTarget))
    {
        if let Some(cursor_transform) = cursors.iter().last() {
            if let Some(player_target_id) = player_target.target.take() {
                commands.entity(player_target_id).despawn();
            }
            let player_target_id = commands
                .spawn(Target::Earth(cursor_transform.translation))
                .id();
            let _ = player_target.target.insert(player_target_id);
        }
    }
}
