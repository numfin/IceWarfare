use avian3d::prelude::{PhysicsDebugPlugin, PhysicsPlugins};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;

pub struct AppInitPlugin {
    pub debug: bool,
}
#[derive(Resource)]
pub struct EnableGizmoDebug;

impl Plugin for AppInitPlugin {
    fn build(&self, app: &mut App) {
        let mut default_plugins = DefaultPlugins.build();
        if self.debug {
            default_plugins = default_plugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            });
            app.insert_resource(WireframeConfig {
                global: true,
                default_color: Color::WHITE.into(),
            })
            .insert_resource(EnableGizmoDebug)
            .add_plugins(PhysicsDebugPlugin::default());
        }
        app.add_plugins(default_plugins)
            .add_plugins(WireframePlugin)
            .add_plugins(PhysicsPlugins::default());
    }
}
