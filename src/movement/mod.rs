use avian3d::prelude::*;
use bevy::prelude::*;

mod sys_move_towards_target;
mod sys_set_move_target;
mod sys_show_destination;
mod sys_stop_on_reaching_target;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sys_show_destination::system)
            .add_systems(Update, sys_stop_on_reaching_target::system)
            .add_systems(Update, sys_set_move_target::system)
            .add_systems(
                PhysicsSchedule,
                sys_move_towards_target::system.before(PhysicsStepSet::First),
            );
    }
}

#[derive(Component, Deref)]
pub struct MoveTarget(pub Vec2);
impl MoveTarget {
    pub fn dir3d(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: 0.0,
            z: self.y,
        }
    }
}
