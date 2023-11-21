use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use boat::BoatPlugin;
use chicken::ChickenPlugin;
use land::LandPlugin;
use main_menu::MainMenuPlugin;

mod boat;
mod chicken;
mod land;
mod main_menu;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    ScoreMenu,
}

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub enum ObjectLocation {
    Land,
    Boat,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "debug".into(),
                resolution: (1280.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(MainMenuPlugin)
        .add_plugins(BoatPlugin)
        .add_plugins(LandPlugin)
        .add_plugins(ChickenPlugin)
        .add_systems(Startup, camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::Rgba {
                red: 0.0,
                green: 204.0,
                blue: 255.0,
                alpha: 0.0,
            }),
        },
        ..default()
    });
}
