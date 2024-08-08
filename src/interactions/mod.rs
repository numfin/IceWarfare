mod sys_on_moving_hit_moving;
mod sys_on_wall_hit;

use bevy::prelude::*;

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sys_on_moving_hit_moving::system)
            .add_systems(Update, sys_on_wall_hit::system);
    }
}
