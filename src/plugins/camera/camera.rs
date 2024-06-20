use bevy::prelude::*;

pub fn system_spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 1., 5.)),
        ..default()
    });
}
