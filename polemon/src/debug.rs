use bevy::prelude::*;
use crate::player::Player;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>()
                ;
        }
    }
}
