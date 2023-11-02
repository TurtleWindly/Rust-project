use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;

use bevy::window::PrimaryWindow;
use boat::BoatPlugin;
use main_menu::MainMenuPlugin;

mod boat;
mod main_menu;

pub struct UiFont(pub Handle<Font>);

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum GameState {
    MainMenu,
    InGame,
    ScoreMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "debug".into(),
                resolution: (640.0, 480.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BoatPlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(PreStartup, load_font)
        .add_systems(Startup, camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands, primary_query: Query<&Window, With<PrimaryWindow>>) {
    let Ok(primay) = primary_query.get_single() else { return; };

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::Rgba { red: 0.0, green: 204.0, blue: 255.0, alpha:  0.0} )},
        transform: Transform { translation: Vec3 { x: primay.width() / 2., y: primay.height() / 2., z:  100.}, ..default()},
        ..default()
    });
}

fn load_font(mut commands: Commands, assets: Res<AssetServer>) {
    let handle: Handle<Font> = assets.load("HackNerdFont.ttf");

    commands.insert_resource(UiFont(handle));
}

