use bevy::prelude::*;

use super::Health;

pub fn system(mut healths: Query<(&mut Transform, &Health)>, time: Res<Time<Virtual>>) {
    for (mut form, health) in &mut healths {
        form.scale.x = form
            .scale
            .x
            .lerp(health.ratio(), time.delta_seconds() * 5.0);
    }
}
