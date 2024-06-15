mod plugins;

use bevy::prelude::*;
use plugins::bot::BotPlugin;
use plugins::camera::GameCameraPlugin;
use plugins::cursor::CursorPlugin;
use plugins::earth::EarthPlugin;
use plugins::health::HealthPlugin;
use plugins::init::AppInitPlugin;
use plugins::physics::PhysicsPlugin;
use plugins::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(AppInitPlugin { debug: true })
        .add_plugins((
            GameCameraPlugin,
            EarthPlugin,
            PlayerPlugin,
            CursorPlugin,
            BotPlugin,
            HealthPlugin,
            PhysicsPlugin,
        ))
        .run();
}
