use bevy::prelude::*;
use bevy_xpbd_3d::components::LinearVelocity;
use bevy_xpbd_3d::plugins::setup::Physics;

use super::components::{Acceleration, PlayerTarget};

pub fn system_move_towards_target(
    mut players: Query<(
        &Transform,
        &mut LinearVelocity,
        &PlayerTarget,
        &Acceleration,
    )>,
    time: Res<Time<Physics>>,
    mut gizmos: Gizmos,
) {
    let delta = time.delta_seconds();
    for (player_transform, mut lin_vel, player_target, accel) in &mut players {
        let Some(target_position) = player_target.0 else {
            continue;
        };
        let player_on_ground = Vec3::new(
            player_transform.translation.x,
            0.0,
            player_transform.translation.z,
        );
        let target_on_ground = Vec3::new(target_position.x, 0.0, target_position.z);

        let dir = target_on_ground - player_on_ground;

        gizmos.ray(player_transform.translation, dir, Color::RED);
        // player_transform.rotation = player_transform.rotation.lerp(
        //     player_transform.looking_to(dir, Vec3::Y).rotation,
        //     delta * 10.,
        // );

        let added_velocity = dir.normalize() * delta * accel.0;

        lin_vel.0 = (lin_vel.0 + added_velocity).clamp_length_max(100.0);
    }
}

pub fn system_accelerate_over_time(
    mut player_accels: Query<(&mut Acceleration, &PlayerTarget)>,
    time: Res<Time<Physics>>,
) {
    let accel_rate = 10.0;
    let max_accel = 30.0;
    let delta = time.delta_seconds() * accel_rate;

    for (mut accel, target) in &mut player_accels {
        if target.0.is_some() {
            accel.0 = (accel.0 + delta).min(max_accel);
        } else {
            accel.0 = (accel.0 - delta).max(0.0);
        }
    }
}

pub fn system_stop_moving_when_reached_target(mut players: Query<(&Transform, &mut PlayerTarget)>) {
    for (player_transform, mut player_target) in &mut players {
        let Some(target_position) = player_target.0 else {
            continue;
        };
        let player_to_target_distance = player_transform.translation.distance(target_position);

        if player_to_target_distance < 1.0 {
            player_target.0.take();
        }
    }
}
