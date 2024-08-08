use core::f32;
use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;
use tiny_bail::cq;

use super::sys_animate_character::{AnimationKind, CharacterAnimations, PlayerReference};
use super::sys_spawn_character::CharacterModel;

pub fn system(
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    models: Query<(&PlayerReference, &CharacterAnimations)>,
    characters: Query<(&CharacterModel, &ExternalForce)>,
) {
    for (character_model, force) in &characters {
        let (player_ref, char_animations) = cq!(models.get(character_model.0));
        let (mut player, mut transitions) = cq!(players.get_mut(player_ref.0));

        let kind = match force.length() {
            ..=0.0 => AnimationKind::Idle,
            ..8.0 => AnimationKind::Walking,
            8.0.. => AnimationKind::Running,
            _ => unreachable!(),
        };
        let anim_kind = cq!(char_animations.get(&kind));
        if transitions
            .get_main_animation()
            .is_some_and(|clip| clip == *anim_kind)
        {
            continue;
        }
        transitions
            .play(&mut player, *anim_kind, Duration::from_millis(200))
            .repeat();
    }
}
