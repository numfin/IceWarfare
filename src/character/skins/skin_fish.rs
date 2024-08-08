use std::collections::HashMap;

use bevy::prelude::*;

use crate::character::sys_animate_character::AnimationKind;
use crate::character::sys_spawn_character::EventSpawnCharacter;

#[derive(Component)]
pub struct SkinFish {
    pub is_controllable: bool,
    pub health: u16,
    pub player_position: Vec3,
}
impl SkinFish {
    pub fn create_spawner(self) -> impl FnMut(Commands) {
        let asset = "fish/fish.glb";

        move |mut cmd: Commands| {
            use AnimationKind::*;
            use GltfAssetLabel::Animation;
            let model_animations = HashMap::from([
                (Idle, Animation(4).from_asset(asset)),
                (Walking, Animation(0).from_asset(asset)),
                (Running, Animation(2).from_asset(asset)),
            ]);
            cmd.trigger(EventSpawnCharacter {
                health: self.health,
                player_position: self.player_position,
                is_controllable: self.is_controllable,
                model_path: GltfAssetLabel::Scene(0).from_asset(asset),
                model_animations,
                model_transform: Transform::from_scale(Vec3::splat(2.0)),
            })
        }
    }
}
