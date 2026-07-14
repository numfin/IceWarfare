pub mod sys_camera_follow_target;
mod sys_spawn_camera;

use avian3d::prelude::*;
use bevy::prelude::*;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, sys_spawn_camera::system)
            .add_systems(
                FixedPostUpdate,
                sys_camera_follow_target::system
                    .after(PhysicsSystems::Last)
                    .before(TransformSystems::Propagate),
            );
    }
}
