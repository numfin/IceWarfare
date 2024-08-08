use avian3d::prelude::*;
use bevy::prelude::*;
use tiny_bail::cq;

use crate::movement::MoveTarget;

use super::sys_spawn_character::CharacterModel;

pub fn system(
    players: Query<(&CharacterModel, &MoveTarget)>,
    mut models: Query<&mut Transform, (With<Handle<Scene>>,)>,
    time: Res<Time<Physics>>,
) {
    for (char_model, move_target) in &players {
        let mut model_t = cq!(models.get_mut(char_model.0));
        let target_dir = model_t.translation - move_target.dir3d().with_y(model_t.translation.y);
        let looking_to = model_t.forward();
        model_t.look_to(
            looking_to.lerp(target_dir.normalize(), time.delta_seconds() * 5.0),
            Vec3::Y,
        );
    }
}
