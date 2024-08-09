use bevy::color::palettes::tailwind;
use bevy::prelude::*;

use super::MoveTarget;

pub fn system(players: Query<(&Transform, &MoveTarget)>, mut gizmos: Gizmos) {
    for (player, target) in &players {
        gizmos.arrow(
            player.translation,
            target.dir3d().with_y(1.0),
            tailwind::RED_500,
        );
    }
}
