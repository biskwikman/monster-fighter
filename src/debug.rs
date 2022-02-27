use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
//TODO how to do #define better or better yet remove on --release
pub const ENABLE_INSPECTOR: bool = false;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if ENABLE_INSPECTOR {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}