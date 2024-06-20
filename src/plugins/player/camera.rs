use bevy::prelude::*;

use crate::plugins::player::Player;

pub fn system_camera_above_player(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    players: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
) {
    for mut camera_t in &mut cameras {
        for player_t in &players {
            let top_view = Vec3::Z * 20.0 + player_t.up() * 20.0 + player_t.translation;
            *camera_t = player_t
                .with_translation(top_view)
                .looking_at(player_t.translation, Vec3::Y);
        }
    }
}
