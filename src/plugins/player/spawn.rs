use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::plugins::health::{Health, HealthBundle, HealthRelation};

use super::components::{Acceleration, Player, PlayerTarget};

pub fn spawn_player(
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
            Name::new("Player"),
            Player,
            HealthRelation { health },
            Acceleration(0.0),
            PlayerTarget(None),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 1.6, 0.5)),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                ..default()
            },
        ))
        .insert((
            RigidBody::Dynamic,
            Collider::cylinder(1.6, 0.6),
            LockedAxes::new()
                .lock_rotation_x()
                .lock_rotation_z()
                .lock_translation_y(),
            LinearDamping(0.3),
            AngularDamping(10.0),
        ));
}
