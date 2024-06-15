use bevy::prelude::*;

#[derive(Component)]
pub enum Target {
    Earth(Vec3),
    #[deprecated]
    Unknown,
}
