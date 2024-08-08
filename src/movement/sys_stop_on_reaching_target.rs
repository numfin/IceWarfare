use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use super::MoveTarget;

pub fn system(
    mut commands: Commands,
    players: Query<(Entity, &Transform, &MoveTarget, &LinearVelocity)>,
) {
    for (id, player_transform, player_target, vel) in &players {
        let target_position = player_target.dir3d().with_y(player_transform.translation.y);
        let player_to_target_distance = player_transform.translation.distance(target_position);

        if player_to_target_distance < 1.0 && vel.length() < 2.0 {
            commands.entity(id).remove::<MoveTarget>();
        }
    }
}
