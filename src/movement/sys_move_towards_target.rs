use avian3d::prelude::ExternalForce;
use bevy::prelude::*;

use super::MoveTarget;

pub fn system(
    mut players: Query<(Entity, &Transform, &mut ExternalForce)>,
    move_targets: Query<&MoveTarget>,
) {
    for (entity, player_transform, mut force) in &mut players {
        let player_on_ground = player_transform.translation.with_y(0.0);
        if let Ok(player_target) = move_targets.get(entity) {
            let dir = player_target.dir3d() - player_on_ground;
            force.set_force(dir * 2.0);
        } else {
            force.set_force(Vec3::ZERO);
        }
    }
}
