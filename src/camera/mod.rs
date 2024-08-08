mod sys_camera_follow_target;
mod sys_spawn_camera;

use avian3d::prelude::*;
use bevy::prelude::*;
pub use sys_camera_follow_target::CameraFollowTarget;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, sys_spawn_camera::system)
            .add_systems(
                PostUpdate,
                sys_camera_follow_target::system
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
