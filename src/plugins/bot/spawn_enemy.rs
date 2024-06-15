use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::plugins::health::{Health, HealthBundle, HealthRelation};

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let health = commands
        .spawn(HealthBundle {
            health: Health::new(1000),
            model: PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 0.2, 0.1)),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 1.5, 0.0)),
                ..default()
            },
        })
        .id();

    commands
        .spawn((
            Name::new("Bot"),
            super::Bot,
            HealthRelation { health },
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 1.6, 0.5)),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::X * 10.0),
                ..default()
            },
        ))
        .insert((
            RigidBody::Dynamic,
            Collider::cylinder(1.6, 0.6),
            LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            LinearDamping(0.1),
            AngularDamping(2.0),
        ));
}
