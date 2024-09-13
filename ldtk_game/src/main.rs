use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_light_2d::prelude::*;

mod collider;
mod goal;
mod player;
mod shop;
mod tourch;

use collider::ColliderPlugin;
use goal::GoalPlugin;
use player::PlayerPlugin;
use shop::ShopPlugin;
use tourch::TourchPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "debug".into(),
                        name: Some("debug".into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((LdtkPlugin, Light2dPlugin))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins((
            PlayerPlugin,
            GoalPlugin,
            ShopPlugin,
            TourchPlugin,
            ColliderPlugin,
        ))
        .insert_resource(LevelSelection::index(0))
        .insert_resource(Gravity(Vec2::new(0.0, 0.0)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tile-based-game.ldtk"),
        ..Default::default()
    });
}
