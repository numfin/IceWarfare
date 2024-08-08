use bevy::prelude::*;

#[derive(Component)]
pub struct CameraFollowTarget;

pub fn system(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    follow_targets: Query<&mut Transform, (With<CameraFollowTarget>, Without<Camera3d>)>,
) {
    for mut camera_t in &mut cameras {
        for player_t in &follow_targets {
            let top_view = Vec3::Z * 20.0 + player_t.up() * 20.0 + player_t.translation;
            *camera_t = player_t
                .with_translation(top_view)
                .looking_at(player_t.translation, Vec3::Y);
        }
    }
}
