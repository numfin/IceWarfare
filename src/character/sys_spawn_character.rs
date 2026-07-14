use avian3d::prelude::*;
use bevy::prelude::*;

// use crate::camera::CameraFollowTarget;
// use crate::controls::IsControllable;
// use crate::health::Health;

use crate::camera::sys_camera_follow_target::CameraFollowTarget;
use crate::character::skins::CharacterSkin;
use crate::controls::IsControllable;

#[derive(Message)]
pub struct PleaseSpawnCharacter {
    pub health: u16,
    pub player_position: Vec3,
    // pub model_path: AssetPath<'static>,
    // pub model_animations: HashMap<CharacterAnimationKind, AssetPath<'static>>,
    // pub model_transform: Transform,
    pub is_controllable: bool,
    pub skin: CharacterSkin,
}

#[derive(Component)]
pub struct CharacterModel(Entity);
impl CharacterModel {
    pub fn new(e: Entity) -> Self {
        Self(e)
    }
    pub fn skin_entity(&self) -> Entity {
        self.0
    }
}

/// This system adds spawns characters when needed
pub fn observer(mut spawn_params: MessageReader<PleaseSpawnCharacter>, mut commands: Commands) {
    for spawn_cmd in spawn_params.read() {
        let mut character = commands.spawn((
            // Health::new(params.health),
            Transform::from_translation(spawn_cmd.player_position),
            spawn_cmd.skin.clone(),
            //
            // ConstantForce::new(0.0, 100.0, 0.0),
            ConstantLocalForce::new(0.0, 0.0, 0.0),
            RigidBody::Dynamic,
            Collider::cylinder(1.0, 0.6),
            LockedAxes::new()
                .lock_rotation_x()
                .lock_rotation_z()
                .lock_rotation_y()
                .lock_translation_y(),
            LinearDamping(0.2),
            Friction::new(0.0),
        ));
        if spawn_cmd.is_controllable {
            character.insert((CameraFollowTarget, IsControllable));
        }
    }
}
