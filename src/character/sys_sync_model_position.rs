use bevy::prelude::*;
use tiny_bail::prelude::cq;

use crate::character::{skins::SkinOfCharacter, sys_spawn_character::CharacterModel};

pub fn system(
    playable_characters: Query<(&CharacterModel, &Transform)>,
    mut player_skins: Query<&mut Transform, (With<SkinOfCharacter>, Without<CharacterModel>)>,
) {
    for (char_model, player_t) in &playable_characters {
        let player_skin_t = player_skins.get_mut(char_model.skin_entity());
        let mut player_skin_t = cq!(player_skin_t);
        player_skin_t.translation.x = player_t.translation.x;
        player_skin_t.translation.z = player_t.translation.z;
    }
}
