use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use land::LandPlugin;
use losemenu::LoseMenu;
use mainmenu::MainMenuPlugin;
use pipe::PipePlugin;
use player::PlayerPlugin;
use scores::{Scores, ScoresPlugin};

mod land;
mod losemenu;
mod mainmenu;
mod pipe;
mod player;
mod scores;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    ScoreMenu,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "debug".into(),
            width: 576.,
            height: 768.,
            ..default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -200.),
            ..default()
        })
        .insert_resource(Scores(0.))
        .add_startup_system_to_stage(StartupStage::PreStartup, load_font)
        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PlayerPlugin)
        .add_plugin(LandPlugin)
        .add_plugin(PipePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(LoseMenu)
        .add_plugin(ScoresPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands, window_des: Res<WindowDescriptor>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = window_des.width / 2.;
    camera.transform.translation.y = window_des.height / 2.;

    commands.spawn_bundle(camera);
}

pub struct UiFont(pub Handle<Font>);

fn load_font(mut commands: Commands, assets: Res<AssetServer>) {
    let handle: Handle<Font> = assets.load("HackNerdFont.ttf");

    commands.insert_resource(UiFont(handle));
}
