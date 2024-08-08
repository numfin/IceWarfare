pub mod skins;
pub mod sys_animate_character;
pub mod sys_spawn_character;
mod sys_sync_model_animation;
mod sys_sync_model_position;
mod sys_sync_model_rotation;

use avian3d::prelude::PhysicsSet;
use bevy::animation::animate_targets;
use bevy::prelude::*;
use skins::skin_brax::SkinBrax;
use skins::skin_fish::SkinFish;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<sys_spawn_character::EventSpawnCharacter>()
            .observe(sys_spawn_character::observer)
            .add_systems(
                Update,
                sys_animate_character::system.before(animate_targets),
            )
            .add_systems(
                PostUpdate,
                (
                    sys_sync_model_position::system,
                    sys_sync_model_rotation::system,
                    sys_sync_model_animation::system,
                )
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(
                Startup,
                (
                    SkinFish {
                        health: 100,
                        player_position: Vec3::Y,
                        is_controllable: true,
                    }
                    .create_spawner(),
                    SkinBrax {
                        health: 100,
                        player_position: Vec3::new(4.0, 1.0, 2.0),
                        is_controllable: false,
                    }
                    .create_spawner(),
                ),
            );
    }
}
