mod sys_spawn_healthbars;
mod sys_sync_health_position;
mod sys_update_health_width;

use avian3d::prelude::PhysicsSet;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

#[derive(Component, Clone, Copy, Debug)]
pub struct Health {
    pub current: u16,
    pub max: u16,
}
impl Health {
    pub fn new(max: u16) -> Self {
        Self { current: max, max }
    }
    pub fn dmg(&mut self, amount: f32) {
        self.current = self.current.saturating_sub(amount as u16);
    }
    pub fn ratio(self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

#[derive(Component)]
pub struct HealthReference(pub Entity);

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sys_update_health_width::system)
            .add_systems(Update, sys_spawn_healthbars::system)
            .add_systems(
                PostUpdate,
                sys_sync_health_position::system
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
