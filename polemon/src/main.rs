use bevy::{prelude::*, window::PresentMode, render::camera::ScalingMode};

use player::PlayerPlugin;
use ascii::AsciiPlugin;
use debug::DebugPlugin;
use titlemap::TitleMapPlugin;

mod player;
mod ascii;
mod debug;
mod titlemap;

pub const RESOLUTION: f32 = 16. / 9.;
pub const TITLE_SIZE: f32 = 0.1;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1366.,
            height: 768.,
            title: "debug".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TitleMapPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.;
    camera.orthographic_projection.bottom = -1.;

    camera.orthographic_projection.right = 1. * RESOLUTION;
    camera.orthographic_projection.left = -1. * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

