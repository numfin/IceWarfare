pub use bevy::prelude::*;

use super::{Health, HealthRelation};

pub fn sync_health_position(
    players: Query<(&Transform, &HealthRelation), Without<Health>>,
    mut healths: Query<&mut Transform, With<Health>>,
) {
    for (p_transform, HealthRelation { health }) in &players {
        if let Ok(mut h_transform) = healths.get_mut(*health) {
            h_transform.translation = p_transform.translation;
            h_transform.translation.y = p_transform.translation.y + 2.0;
        }
    }
}
