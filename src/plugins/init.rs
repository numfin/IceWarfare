use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};

pub struct AppInitPlugin {
    pub debug: bool,
}
impl Plugin for AppInitPlugin {
    fn build(&self, app: &mut App) {
        let mut _app = if self.debug {
            app.add_plugins((
                DefaultPlugins.set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                }),
                WireframePlugin,
            ))
            .insert_resource(WireframeConfig {
                global: true,
                default_color: Color::WHITE.into(),
            })
        } else {
            app.add_plugins(DefaultPlugins)
        };
        app.add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default());
    }
}
