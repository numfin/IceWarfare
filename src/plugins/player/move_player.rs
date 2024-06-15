use bevy::prelude::*;
use bevy_xpbd_3d::components::LinearVelocity;
use bevy_xpbd_3d::plugins::setup::Physics;

use crate::plugins::target::Target;

use super::PlayerTargetRelation;

pub fn move_player(
    mut commands: Commands,
    mut players: Query<(
        &mut Transform,
        &mut LinearVelocity,
        &mut PlayerTargetRelation,
    )>,
    targets: Query<&Target>,
    time: Res<Time<Physics>>,
) {
    let delta = time.delta_seconds();
    for (mut player_transform, mut lin_vel, mut player_target) in &mut players {
        let Some(target_id) = player_target.target else {
            continue;
        };
        let Ok(target) = targets.get(target_id) else {
            continue;
        };

        if let Target::Earth(target_position) = target {
            let player_to_target_distance = player_transform.translation.distance(*target_position);
            let dir = Vec3::new(target_position.x, 0.0, target_position.z)
                - Vec3::new(
                    player_transform.translation.x,
                    0.0,
                    player_transform.translation.z,
                );

            player_transform.rotation = player_transform.rotation.lerp(
                player_transform.looking_to(dir, Vec3::Y).rotation,
                delta * 10.,
            );

            let accel = player_transform.forward().normalize() * delta * 30.;
            lin_vel.0 = (lin_vel.0 + accel).clamp_length_max(100.0);
            if player_to_target_distance < 5.0 {
                player_target.target.take();
                commands.entity(target_id).despawn();
            }
        }
    }
}
