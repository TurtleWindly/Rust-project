use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_light_2d::prelude::*;
use bevy_mod_picking::prelude::*;

mod collider;
mod goal;
mod inventory;
mod player;
mod shop;
mod tourch;

use collider::ColliderPlugin;
use goal::GoalPlugin;
use inventory::InventoryPlugin;
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
        .add_plugins((LdtkPlugin, Light2dPlugin, DefaultPickingPlugins))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins((
            PlayerPlugin,
            InventoryPlugin,
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
    // Make object pickable
    commands.spawn((PickableBundle::default(), PbrBundle::default()));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tile-based-game.ldtk"),
        ..Default::default()
    });
}
