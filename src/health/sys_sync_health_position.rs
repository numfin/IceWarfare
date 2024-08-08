pub use bevy::prelude::*;
use tiny_bail::c;

use super::HealthReference;

pub fn system(
    players: Query<(&Transform, &HealthReference)>,
    mut healths: Query<&mut Transform, Without<HealthReference>>,
) {
    for (p_transform, HealthReference(health)) in &players {
        let mut h_transform = c!(healths.get_mut(*health));
        h_transform.translation = p_transform.translation;
        h_transform.translation.y = p_transform.translation.y + 4.0;
    }
}
