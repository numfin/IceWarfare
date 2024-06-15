use crate::plugins::earth::spawn_earth::Ground;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn update_cursor_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    earths: Query<&Transform, With<Ground>>,
    mut cursors: Query<&mut Transform, (With<super::GameCursor>, Without<Ground>)>,
) {
    for (camera, camera_transform) in &cameras {
        for window in &windows {
            if let Some(ray) = window.cursor_position().and_then(|viewport_position| {
                camera.viewport_to_world(camera_transform, viewport_position)
            }) {
                for ground in &earths {
                    let distance =
                        ray.intersect_plane(ground.translation, Plane3d::new(ground.up().into()));
                    if let Some(distance) = distance {
                        let coord = camera_transform.translation() + ray.direction * distance;
                        for mut target in &mut cursors {
                            target.translation = coord;
                        }
                    }
                }
            }
        }
    }
}
