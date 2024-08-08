use bevy::color::palettes::tailwind;
use bevy::prelude::*;

use super::{Health, HealthReference};

pub fn system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    added_healths: Query<(Entity, &Health), Added<Health>>,
) {
    for (entity, ..) in &added_healths {
        let health_model = commands
            .spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 0.2, 0.1)),
                material: materials.add(StandardMaterial {
                    base_color: tailwind::RED_500.into(),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 1.5, 0.0)),
                ..default()
            })
            .id();
        commands
            .entity(entity)
            .insert(HealthReference(health_model));
    }
}
