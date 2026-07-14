use std::f32::consts::PI;

use bevy::{
    light::CascadeShadowConfigBuilder, post_process::auto_exposure::AutoExposure, prelude::*,
};

pub fn system(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0., 1., 5.)),
        AutoExposure::default(),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0))
            .with_rotation(Quat::from_rotation_y(-PI / 4.)),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..Default::default()
        }
        .build(),
    ));
}
