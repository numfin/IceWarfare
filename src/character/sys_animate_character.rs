use std::collections::HashMap;
use std::time::Duration;

use bevy::prelude::*;
use strum::{EnumIs, EnumIter};
use tiny_bail::cq;

#[derive(Hash, PartialEq, Eq, EnumIter, EnumIs, Debug)]
pub enum AnimationKind {
    Idle,
    Walking,
    Running,
}

#[derive(Component, Deref)]
pub struct CharacterAnimations(HashMap<AnimationKind, AnimationNodeIndex>);
impl CharacterAnimations {
    pub fn new(node_collection: HashMap<AnimationKind, AnimationNodeIndex>) -> Self {
        Self(node_collection)
    }
}

/// For referencing player that used for each model
#[derive(Component)]
pub struct PlayerReference(pub Entity);

pub fn system(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    scenes: Query<(Entity, &CharacterAnimations, &Handle<AnimationGraph>)>,
    parents: Query<&Parent>,
) {
    for (player_entity, mut player) in &mut players {
        for player_parent in parents.iter_ancestors(player_entity) {
            let (model_entity, model_animations, graph) =
                cq!(scenes.iter().find(|(id, ..)| player_parent == *id));
            let anim_index = cq!(model_animations.get(&AnimationKind::Idle));

            let mut transitions = AnimationTransitions::new();
            transitions
                .play(&mut player, *anim_index, Duration::ZERO)
                .repeat();
            commands
                .entity(player_entity)
                .insert((transitions, graph.clone()));
            commands
                .entity(model_entity)
                .insert(PlayerReference(player_entity));
        }
    }
}
