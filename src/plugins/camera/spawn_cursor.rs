use bevy::prelude::*;

pub fn spawn_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        super::GameCursor,
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            transform: Transform::default(),
            ..default()
        },
    ));
}
