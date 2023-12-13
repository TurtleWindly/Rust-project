use bevy::prelude::*;

use crate::GameState;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_start_animation)
            .add_systems(Update, start_animation.run_if(in_state(GameState::Game)))
            .add_systems(Update, start_game.run_if(in_state(GameState::MainMenu)));
    }
}

#[derive(Component)]
struct Rectangle;

fn setup_start_animation(
    mut commands: Commands,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.single();
    commands
        .spawn((
            Rectangle,
            Name::new("Rectangle Animation"),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    custom_size: Some(Vec2::new(window.width(), window.height())),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: window.width() / 3.,
                        z: 99.,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Logo"),
                SpriteBundle {
                    texture: asset_server.load("Logo.png"),
                    transform: Transform {
                        translation: Vec3 {
                            x: -180.,
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn start_animation(
    mut commands: Commands,
    mut query: Query<(&mut Transform, Entity), With<Rectangle>>,
) {
    if let Ok((mut transform, entity)) = query.get_single_mut() {
        transform.translation.x += 7.;
        if transform.translation.x > 1500. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn start_game(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.any_pressed([KeyCode::Space, KeyCode::Up]) {
        commands.insert_resource(NextState(Some(GameState::Game)));
    }
}
