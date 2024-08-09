use bevy::prelude::*;
use tiny_bail::c;

use super::sys_spawn_healthbars::HealthModel;
use super::{Health, HealthReference};

pub fn system(
    healths: Query<(&Health, &HealthReference)>,
    mut health_models: Query<&mut Transform, With<HealthModel>>,
    time: Res<Time<Virtual>>,
) {
    for (health, health_ref) in &healths {
        let mut health_model = c!(health_models.get_mut(health_ref.0));
        health_model.scale.x = health_model
            .scale
            .x
            .lerp(health.ratio(), time.delta_seconds() * 5.0);
    }
}
