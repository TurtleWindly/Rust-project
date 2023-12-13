use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod dino;
mod castus;
mod score;
mod start_menu;
mod background;

use start_menu::StartMenuPlugin;
use background::BackGroundPlugin;
use dino::DinoPlugin;
use castus::CastusPlugin;
use score::ScorePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    ScoreMenu,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "debug".into(),
                resolution: (1280.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins((StartMenuPlugin, BackGroundPlugin, DinoPlugin, CastusPlugin, ScorePlugin))
        .add_systems(Startup, camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::Rgba {
                red: 255.,
                green: 255.,
                blue: 255.,
                alpha: 0.,
            }),
        },
        ..default()
    });
}

