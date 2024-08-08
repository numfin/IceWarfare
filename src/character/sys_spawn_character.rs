use std::collections::HashMap;

use avian3d::prelude::*;
use bevy::asset::AssetPath;
use bevy::prelude::*;
use strum::IntoEnumIterator;
use tiny_bail::c;

use crate::camera::CameraFollowTarget;
use crate::controls::IsControllable;
use crate::health::Health;

use super::sys_animate_character::{AnimationKind, CharacterAnimations};

#[derive(Event)]
pub struct EventSpawnCharacter {
    pub health: u16,
    pub player_position: Vec3,
    pub model_path: AssetPath<'static>,
    pub model_animations: HashMap<AnimationKind, AssetPath<'static>>,
    pub model_transform: Transform,
    pub is_controllable: bool,
}

#[derive(Component)]
pub struct CharacterModel(pub Entity);

pub fn observer<'a>(
    trigger: Trigger<EventSpawnCharacter>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut all_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let params = trigger.event();
    let mut graph = AnimationGraph::new();
    let mut animation_node_store = HashMap::new();

    for animation_kind in AnimationKind::iter() {
        let anim_path = c!(params.model_animations.get(&animation_kind));
        let node_index = graph.add_clip(asset_server.load(anim_path), 1.0, graph.root);
        animation_node_store.insert(animation_kind, node_index);
    }
    let animation_graph = all_graphs.add(graph);
    let character_animations = CharacterAnimations::new(animation_node_store);

    let character_model = commands
        .spawn((
            SceneBundle {
                scene: asset_server.load(&params.model_path),
                transform: params.model_transform,
                ..Default::default()
            },
            character_animations,
            animation_graph,
        ))
        .id();

    let mut character = commands.spawn((
        Health::new(params.health),
        CharacterModel(character_model),
        // Physics
        TransformBundle::from_transform(Transform::from_translation(params.player_position)),
        ExternalForce::default(),
        RigidBody::Dynamic,
        Collider::cylinder(1.0, 0.6),
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        LinearDamping(0.2),
        Friction::new(0.0),
    ));
    if params.is_controllable {
        character.insert((CameraFollowTarget, IsControllable));
    }
}
