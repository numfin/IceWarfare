use bevy::{
    gltf::{GltfLoaderSettings, convert_coordinates::GltfConvertCoordinates},
    prelude::*,
};

pub fn spawn(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) -> Entity {
    let asset_path = "fish/fish.glb";

    let (animation_graph, nodes) = AnimationGraph::from_clips(
        [
            GltfAssetLabel::Animation(0).from_asset(asset_path),
            GltfAssetLabel::Animation(1).from_asset(asset_path),
            GltfAssetLabel::Animation(2).from_asset(asset_path),
        ]
        .map(|asset| asset_server.load(asset)),
    );
    let graph_handle = animation_graphs.add(animation_graph);
    let animations_to_play = super::AnimationToPlay {
        graph_handle,
        nodes,
    };

    let scene_handle = asset_server.load_with_settings(
        GltfAssetLabel::Scene(0).from_asset(asset_path),
        |settings: &mut GltfLoaderSettings| {
            settings.convert_coordinates = Some(GltfConvertCoordinates {
                rotate_meshes: true,
                rotate_scene_entity: true,
            });
        },
    );

    commands
        .spawn((
            SceneRoot(scene_handle),
            Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::Y * 0.4),
            animations_to_play,
        ))
        .id()
}
// fn spawn_model() -> impl Fn(DeferredWorld, HookContext) {
//     // let model_animations = HashMap::from([
//     //     (Idle, Animation(4).from_asset(asset)),
//     //     (Walking, Animation(0).from_asset(asset)),
//     //     (Running, Animation(2).from_asset(asset)),
//     // ]);
//     // cmd.trigger(EventSpawnCharacter {
//     //     health: self.health,
//     //     player_position: self.player_position,
//     //     is_controllable: self.is_controllable,
//     //     model_path: GltfAssetLabel::Scene(0).from_asset(asset),
//     //     model_animations,
//     //     model_transform: Transform::from_scale(Vec3::splat(2.0)),
//     // });
//     |world, hook| {
//         let system_id = world.register_system(spawn_model_system);
//         if let Err(err) = world.run_system(system_id) {
//             tracing::error!(?err);
//         }
//     }
// }
