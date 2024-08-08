use bevy::prelude::*;
use tiny_bail::cq;

use super::sys_spawn_character::CharacterModel;

pub fn system(
    players: Query<(&CharacterModel, &Transform)>,
    mut models: Query<&mut Transform, (With<Handle<Scene>>, Without<CharacterModel>)>,
) {
    for (char_model, player_t) in &players {
        let mut model_t = cq!(models.get_mut(char_model.0));
        model_t.translation = player_t.translation;
    }
}
