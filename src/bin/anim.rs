use bevy::{
    color::palettes::basic::WHITE,
    dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin},
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    scene::SceneInstanceReady,
};
use tiny_bail::{or_continue_quiet, or_return_quiet};

/// Initializes the app.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevydev".into(),
                position: WindowPosition::At(IVec2::new(1500, 100)),
                focused: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins((MeshPickingPlugin, DebugPickingPlugin))
        .insert_resource(DebugPickingMode::Normal)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, change_after_time)
        .insert_resource(GlobalAmbientLight {
            color: WHITE.into(),
            brightness: 100.0,
            ..default()
        })
        .run();
}

#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    nodes: Vec<AnimationNodeIndex>,
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 5.0, 13.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 10_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 8.0, 13.0),
    ));

    let (animation_graph, nodes) = AnimationGraph::from_clips(
        [
            GltfAssetLabel::Animation(0).from_asset("fish/fish.glb"),
            GltfAssetLabel::Animation(1).from_asset("fish/fish.glb"),
            GltfAssetLabel::Animation(2).from_asset("fish/fish.glb"),
        ]
        .map(|asset| asset_server.load(asset)),
    );

    let graph_handle = animation_graphs.add(animation_graph);

    let animations_to_play = AnimationToPlay {
        graph_handle,
        nodes,
    };

    commands
        .spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("fish/fish.glb"))),
            Transform::from_scale(Vec3::splat(3.0)),
            animations_to_play,
        ))
        .observe(play_animation_when_ready);

    commands.spawn((
        Mesh3d(meshes.add(Circle::new(7.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
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

fn change_after_time(
    t: Res<Time>,
    children: Query<&Children>,
    animations: Query<(Entity, &AnimationToPlay)>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for (entity, anims) in animations.iter() {
        for child in children.iter_descendants(entity) {
            let mut player = or_continue_quiet!(players.get_mut(child));

            let i = t.elapsed_secs() % anims.nodes.len() as f32;
            let i = i as usize;

            for node in &anims.nodes {
                let is_active = i.is_multiple_of(node.index());
                if !player.is_playing_animation(*node) && is_active {
                    player.play(*node).repeat();
                }
                if let Some(anim) = player.animation_mut(*node) {
                    let weight = if is_active { 1.0 } else { Default::default() };
                    anim.set_weight(weight);
                }
            }
        }
    }
}
