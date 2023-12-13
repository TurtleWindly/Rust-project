use bevy::prelude::*;

use crate::GameState;

pub struct BackGroundPlugin;

impl Plugin for BackGroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, background_setup)
            .add_systems(Update, background_moving.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::ScoreMenu), background_reset);
    }
}

const IMAGE_WIDTH: f32 = 2400.;

#[derive(Component)]
struct BackGround;

fn background_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("background.png");

    for index in 0..=1 {
        commands.spawn((
            BackGround,
            Name::new("Background"),
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3 {
                        x: IMAGE_WIDTH * index as f32,
                        y: -250.,
                        z: 1.,
                    },
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn background_moving(mut query: Query<&mut Transform, With<BackGround>>) {
    for mut bg in query.iter_mut() {
        if bg.translation.x == -IMAGE_WIDTH {
            bg.translation.x = IMAGE_WIDTH;
        }
        bg.translation.x -= 10.;
    }
}

fn background_reset(mut query: Query<&mut Transform, With<BackGround>>) {
    for (index, mut bg) in query.iter_mut().enumerate() {
        bg.translation.x = IMAGE_WIDTH * index as f32;
    }
}
