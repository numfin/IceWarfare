use avian3d::prelude::*;
use bevy::prelude::*;
use tiny_bail::or_continue_quiet;

use crate::{
    character::{skins::SkinOfCharacter, sys_spawn_character::CharacterModel},
    movement::MoveTarget,
};

pub fn system(
    playable_characters: Query<(&CharacterModel, &MoveTarget)>,
    mut player_skins: Query<&mut Transform, With<SkinOfCharacter>>,
    time: Res<Time<Physics>>,
) {
    for (char_model, move_target) in &playable_characters {
        let skin_t = player_skins.get_mut(char_model.skin_entity());
        let mut skin_t = or_continue_quiet!(skin_t);

        let final_point = move_target.dir3d().with_y(skin_t.translation.y);
        let current_point = skin_t.translation;

        let target_dir = current_point - final_point;
        let currently_looking_to = skin_t.forward();
        dbg!(target_dir);
        dbg!(currently_looking_to);

        // info!(?target_dir);

        skin_t.look_to(
            // currently_looking_to.lerp(target_dir.normalize(), time.delta_secs() * 15.0),
            target_dir.normalize(),
            // currently_looking_to,
            Vec3::Y,
        );
    }
}
