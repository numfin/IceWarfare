mod spawn_enemy;
use bevy::prelude::*;

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy::spawn_enemy);
    }
}

#[derive(Component)]
pub struct Bot;
