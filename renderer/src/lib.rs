mod player;
mod projectiles;
mod bot;

use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use lightyear::client::interpolation::VisualInterpolationPlugin;
use lightyear::prelude::client::{Interpolated, Predicted};
use lightyear::prelude::server::ReplicationTarget;

pub struct RendererPlugin;


/// Convenient for filter for entities that should be visible
/// Works either on the client or the server
#[derive(QueryFilter)]
pub struct VisibleFilter {
    a: Or<(
        With<Predicted>,
        With<Interpolated>,
        With<ReplicationTarget>,
    )>,
}

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        // PLUGINS
        // TODO: add option to disable inspector
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        app.add_plugins(bot::BotPlugin);
        app.add_plugins(player::PlayerPlugin);
        app.add_plugins(projectiles::ProjectilesPlugin);
        app.insert_resource(AmbientLight {
            brightness: 0.0,
            ..default()
        });

        #[cfg(feature = "client")]
        // we use Position and Rotation as primary source of truth, so no need to sync changes
        // from Transform->Pos, just Pos->Transform.
        // Also we apply visual interpolation on Transform, but that's just for visuals, we want the real
        // value to still come from Position/Rotation
        app.insert_resource(avian3d::sync::SyncConfig {
            transform_to_position: false,
            position_to_transform: true,
            ..default()
        });
        app.add_plugins(VisualInterpolationPlugin::<Transform>::default());

        // SYSTEMS
        // TODO: separate client renderer from server renderer? The features cfg are not enough
        // on the server, the camera doesn't follow a player
        #[cfg(not(feature = "client"))]
        app.add_systems(Startup, init);
    }
}


#[cfg(not(feature = "client"))]
fn init(mut commands: Commands) {
    dbg!("ADD CAM");
    commands.spawn(Camera3d::default());
}
