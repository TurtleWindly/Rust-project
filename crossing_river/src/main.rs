use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use boat::BoatPlugin;
use chicken::ChickenPlugin;
use land::{LandPlugin, LandSetupSet, AfterSpawnObjectSet};
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

#[derive(Component, Default)]
pub struct ObjectLayout {
    had_setup: bool,
}

#[derive(Component)]
pub struct Object {
    located: bool,
}

impl Default for Object {
    fn default() -> Self {
        Object { located: false }
    }
}

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
        .add_systems(OnEnter(GameState::Game), object_layout_setup.in_set(LandSetupSet))
        .add_systems(Update, add_children_object_layout.run_if(have_layout_and_objects))
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

fn object_layout_setup(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            background_color: Color::BLACK.into(),
            style: Style {
                width: Val::Percent(100.),
                ..default()
            },
            transform: Transform {
                translation: Vec3 { x: 0., y: 0.,  z: 4.,},
                ..default()
            },
            ..default()
        },
        ObjectLayout::default(),
        Name::new("Object Layout"),
    ));
}

fn have_layout_and_objects(layout: Query<&ObjectLayout>, objects_query: Query<&Name, (With<Object>, Without<ObjectLayout>)>) -> bool {
    if let Ok(layout) = layout.get_single() {
        if layout.had_setup { return false; }
        // NOTE: Hard code Number of object in one level
        let mut index: u32 = 1;
        for _ in objects_query.iter() {
            index -= 1;
        }
        return index == 0;
    }
    return false;
}

fn add_children_object_layout(mut commands: Commands, mut object_layout: Query<(Entity, &mut ObjectLayout), With<ObjectLayout>>, objects_query: Query<Entity, (With<Object>, Without<ObjectLayout>)>) {
    for (parent, mut layout) in object_layout.iter_mut() {
        layout.had_setup = true;
        for object in objects_query.iter() {
            commands.entity(parent).push_children(&[object]);
        }
    }
 }
