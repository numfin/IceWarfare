use avian3d::prelude::{PhysicsDebugPlugin, PhysicsPlugins};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::remote::RemotePlugin;
use bevy::remote::http::RemoteHttpPlugin;
use bevy::render::RenderPlugin;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};

pub struct AppInitPlugin {
    pub debug: bool,
}

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
                default_color: Color::WHITE,
            })
            .add_plugins(PhysicsDebugPlugin)
            .add_plugins(RemotePlugin::default())
            .add_plugins(RemoteHttpPlugin::default());
        }
        app.add_plugins(default_plugins)
            .add_plugins(WireframePlugin::default())
            .add_plugins(PhysicsPlugins::default());
    }
}
