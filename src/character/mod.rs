pub mod skins;
pub mod sys_spawn_character;
// mod sys_sync_model_animation;
mod sys_sync_model_position;
mod sys_sync_model_rotation;

use avian3d::prelude::PhysicsSystems;
use bevy::prelude::*;

use crate::character::{skins::CharacterSkinPlugin, sys_spawn_character::PleaseSpawnCharacter};
// use skins::skin_brax::SkinBrax;
// use skins::skin_fish::SkinFish;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PleaseSpawnCharacter>()
            .add_plugins(CharacterSkinPlugin)
            .add_systems(Update, sys_spawn_character::observer)
            .add_systems(Startup, spawn_some)
            // .add_systems(
            //     Update,
            //     sys_animate_character::system.before(animate_targets),
            // )
            .add_systems(
                PostUpdate,
                (
                    sys_sync_model_position::system,
                    sys_sync_model_rotation::system,
                    //         // sys_sync_model_animation::system,
                )
                    .after(PhysicsSystems::Last)
                    .before(TransformSystems::Propagate),
            );
    }
}

fn spawn_some(mut commands: Commands) {
    commands.write_message(PleaseSpawnCharacter {
        health: 100,
        player_position: Vec3::splat(0.0),
        is_controllable: true,
        skin: skins::CharacterSkin::Fish,
    });
}
