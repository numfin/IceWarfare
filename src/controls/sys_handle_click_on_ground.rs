use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::prelude::*;

use super::sys_add_mouse_listener::ActionMouse;
use super::EventPlayerAction;

pub fn system(
    mut ev_writer: EventWriter<EventPlayerAction>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_actions: Query<&ActionState<ActionMouse>>,
) {
    let is_clicked = mouse_actions
        .iter()
        .filter(|action| action.just_pressed(&ActionMouse::Click))
        .count()
        > 0;
    if is_clicked {
        for (camera, camera_transform) in &cameras {
            for window in &windows {
                if let Some(ray) = window
                    .cursor_position()
                    .and_then(|viewport| camera.viewport_to_world(camera_transform, viewport))
                {
                    let distance = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y));
                    if let Some(distance) = distance {
                        let coord = camera_transform.translation() + ray.direction * distance;
                        ev_writer.send(EventPlayerAction::ClickOnGround(coord.xz()));
                    }
                }
            }
        }
    }
}
