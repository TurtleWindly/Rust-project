use bevy::prelude::*;
use debug::DebugPlugin;
use sorting::SortingPlugin;
use sorting_algorithms::SortingAlgorithms;

mod debug;
mod sorting;
mod sorting_algorithms;
mod array;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "debug".into(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(SortingPlugin)
        .add_plugin(SortingAlgorithms)
        .run();
}

fn setup_camera(mut commands: Commands, windows_des: Res<WindowDescriptor>) {
    let mut camera = Camera2dBundle::default();

    camera.transform.translation.x = windows_des.width / 2.;
    camera.transform.translation.y = windows_des.height / 2.;

    commands.spawn_bundle(camera);
}
