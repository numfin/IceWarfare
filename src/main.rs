mod camera;
mod character;
mod controls;
mod earth;
mod health;
mod init;
mod interactions;
mod movement;
mod physics;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(init::AppInitPlugin { debug: false })
        .add_plugins((
            controls::ControlPlugin,
            movement::MovementPlugin,
            physics::PhysicsPlugin,
            character::CharacterPlugin,
            camera::GameCameraPlugin,
            earth::EarthPlugin,
            health::HealthPlugin,
            interactions::InteractionPlugin,
        ))
        .run();
}
