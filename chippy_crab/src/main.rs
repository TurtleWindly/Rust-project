use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use land::LandPlugin;
use pipe::PipePlugin;
use player::PlayerPlugin;

mod land;
mod pipe;
mod player;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "debug".into(),
            width: 576.,
            height: 768.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(LandPlugin)
        .add_plugin(PipePlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands, window_des: Res<WindowDescriptor>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = window_des.width / 2.;
    camera.transform.translation.y = window_des.height / 2.;

    commands.spawn_bundle(camera);
}
