use bevy::prelude::*;
use bevy_xpbd_3d::components::{CoefficientCombine, Restitution, RigidBody};
use bevy_xpbd_3d::plugins::collision::Collider;

static EARTH_SIZE: f32 = 100.0;

pub fn system_spawn_bounds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_ground(
        "Ground",
        EARTH_SIZE,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    create_wall(
        "Wall_Top",
        (EARTH_SIZE, 1.0),
        (0.0, -EARTH_SIZE / 2.0 - 0.5),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    create_wall(
        "Wall_Bottom",
        (EARTH_SIZE, 1.0),
        (0.0, EARTH_SIZE / 2.0 + 0.5),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    create_wall(
        "Wall_Left",
        (1.0, EARTH_SIZE),
        (-EARTH_SIZE / 2.0 - 0.5, 0.0),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    create_wall(
        "Wall_Right",
        (1.0, EARTH_SIZE),
        (EARTH_SIZE / 2.0 + 0.5, 0.0),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::new(EARTH_SIZE, 1.0, EARTH_SIZE)),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::WHITE,
    //             ..default()
    //         }),
    //         transform: Transform::from_translation(Vec3::NEG_Y),
    //         ..default()
    //     },
    //     RigidBody::Static,
    //     Collider::cuboid(EARTH_SIZE, 1.0, EARTH_SIZE),
    // ));
}

#[derive(Component)]
pub struct Ground;

fn create_ground(
    name: &'static str,
    size: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Name::new(name),
            Ground,
            PbrBundle {
                mesh: meshes.add(Cuboid::new(EARTH_SIZE, 1.0, EARTH_SIZE)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::NEG_Y * 2.0),
                ..default()
            },
        ))
        .insert((RigidBody::Static, Collider::cuboid(size, 1.0, size)));
}

#[derive(Component)]
pub struct Wall;

fn create_wall(
    name: &'static str,
    (x, z): (f32, f32),
    (dx, dz): (f32, f32),
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let wall_h = 2.0;
    commands
        .spawn((
            Name::new(name),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(x, wall_h, z)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(dx, 0.0, dz)),
                ..default()
            },
            Wall,
        ))
        .insert((
            RigidBody::Static,
            Collider::cuboid(x, wall_h, z),
            Restitution::new(0.1).with_combine_rule(CoefficientCombine::Multiply),
        ));
}
