mod sync_health_position;
mod update_health_width;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::PhysicsSet;

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
pub struct HealthRelation {
    pub health: Entity,
}

#[derive(Bundle)]
pub struct HealthBundle {
    pub health: Health,
    pub model: PbrBundle,
}

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_health_width::update_health_width)
            .add_systems(
                PostUpdate,
                sync_health_position::sync_health_position
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
