use std::f32::consts::PI;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

pub fn system(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 1., 5.)),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 2.0, 0.0))
            .with_rotation(Quat::from_rotation_y(-PI / 4.)),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });
}
