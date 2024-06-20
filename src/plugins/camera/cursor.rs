use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::action_state::ActionState;

use super::mouse::MouseAction;

#[derive(Event)]
pub struct EventClickGround(pub Vec3);

pub fn system_handle_click_on_ground(
    mut ev_writer: EventWriter<EventClickGround>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_actions: Query<&ActionState<MouseAction>>,
) {
    let is_clicked = mouse_actions
        .iter()
        .filter(|action| action.just_pressed(&MouseAction::SetTarget))
        .count()
        > 0;
    if is_clicked {
        for (camera, camera_transform) in &cameras {
            for window in &windows {
                if let Some(ray) = window
                    .cursor_position()
                    .and_then(|viewport| camera.viewport_to_world(camera_transform, viewport))
                {
                    let distance = ray.intersect_plane(Vec3::ZERO, Plane3d::new(Vec3::Y));
                    if let Some(distance) = distance {
                        let coord = camera_transform.translation() + ray.direction * distance;
                        ev_writer.send(EventClickGround(coord));
                    }
                }
            }
        }
    }
}
