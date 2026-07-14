pub mod skin_fish;

use bevy::{prelude::*, scene::SceneInstanceReady};
use tiny_bail::{or_continue_quiet, or_return, or_return_quiet};

use crate::character::sys_spawn_character::CharacterModel;

// #[derive(Component, Debug)]
// #[relationship(relationship_target = )]
// pub struct SkinOf(Entity);

#[derive(Component, Clone)]
pub enum CharacterSkin {
    Fish,
}
#[derive(Component)]
pub struct SkinOfCharacter;

pub struct CharacterSkinPlugin;
impl Plugin for CharacterSkinPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_skin_spawn);
    }
}

#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    nodes: Vec<AnimationNodeIndex>,
}

fn play_animation_when_ready(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let animation_to_play = or_return_quiet!(animations_to_play.get(scene_ready.entity));
    for child in children.iter_descendants(scene_ready.entity) {
        let mut player = or_continue_quiet!(players.get_mut(child));
        player.play(animation_to_play.nodes[0]).repeat();

        commands
            .entity(child)
            .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
    }
}

fn on_skin_spawn(
    trigger: On<Add, CharacterSkin>,
    skins: Query<&CharacterSkin>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let skin = or_return!(skins.get(trigger.entity));
    let spawned_model_entity = match skin {
        CharacterSkin::Fish => skin_fish::spawn(&mut commands, asset_server, animation_graphs),
    };

    or_return!(commands.get_entity(trigger.entity))
        .insert(CharacterModel::new(spawned_model_entity));

    or_return!(commands.get_entity(spawned_model_entity))
        .insert(SkinOfCharacter)
        .observe(play_animation_when_ready);
}
