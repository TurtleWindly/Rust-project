use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{scores::Scores, GameState};

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_pipes))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(pipes_move.before("check pipe"))
                    .with_system(check_pipe_go_off_sceen.label("check pipe")),
            )
            .add_system_set(SystemSet::on_exit(GameState::ScoreMenu).with_system(despawn_pipes));
    }
}

const SPACE_BETWEEN_PIPES: f32 = 300.;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PipeBot;

#[derive(Component)]
pub struct PipeTop;

// Spawn first time position pipes
fn spawn_pipes(
    mut commands: Commands,
    window_des: Res<WindowDescriptor>,
    assest: Res<AssetServer>,
) {
    // Pairs of pipes
    let pipes_will_spawn: usize = 10;
    let pipe_width: f32 = 80.;
    let free_space: f32 = 200.;

    let mut pipe_list: Vec<SpriteBundle> = Default::default();

    let mut rng = thread_rng();

    // Bot
    for index in 0..pipes_will_spawn {
        let random_heigth = rng.gen_range(80.0..(window_des.height - free_space - 100.));

        pipe_list.push(SpriteBundle {
            texture: assest.load("pipe_body.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(pipe_width, random_heigth)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    window_des.width + (index as f32 * SPACE_BETWEEN_PIPES),
                    random_heigth / 2.,
                    1.,
                ),
                ..default()
            },
            ..default()
        });
    }

    // Top
    for index in 0..pipes_will_spawn {
        let pipe_bot_height = pipe_list[index].sprite.custom_size.unwrap().y;
        let pipe_top_height = window_des.height - pipe_bot_height + free_space;

        pipe_list.push(SpriteBundle {
            texture: assest.load("pipe_body.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(pipe_width, pipe_top_height)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    window_des.width + (index as f32 * SPACE_BETWEEN_PIPES),
                    pipe_bot_height + free_space + (pipe_top_height / 2.),
                    1.,
                ),
                ..default()
            },
            ..default()
        });
    }

    for (index, pipe) in pipe_list.into_iter().enumerate() {
        let pipe_width = pipe.sprite.custom_size.unwrap().x;
        let pipe_height = pipe.sprite.custom_size.unwrap().y;

        let entity = commands
            .spawn_bundle(pipe)
            .insert(Pipe)
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::cuboid(pipe_width / 2., pipe_height / 2.))
            .id();

        if index < pipes_will_spawn / 2 {
            commands.entity(entity).insert(PipeBot);
        } else {
            commands.entity(entity).insert(PipeTop);
        }
    }
}

fn pipes_move(mut query: Query<&mut Transform, With<Pipe>>) {
    let pipes_speed: f32 = 2.;
    for mut trasform in query.iter_mut() {
        trasform.translation.x -= pipes_speed;
    }
}

fn check_pipe_go_off_sceen(
    mut query: Query<(&mut Transform, &Sprite), With<Pipe>>,
    mut scores: ResMut<Scores>,
) {
    let mut farest: f32 = 0.;
    let mut current_pos: f32 = 0.;
    for (transform, sprite) in query.iter() {
        // Check for pipe go off screen
        if transform.translation.x < 0. - sprite.custom_size.unwrap().x {
            scores.0 += 0.5;

            current_pos = transform.translation.x;

            // Find the farest position
            for (transform, _) in query.iter() {
                if transform.translation.x > farest {
                    farest = transform.translation.x;
                }
            }
        }
    }

    for (mut transform, _) in query.iter_mut() {
        if transform.translation.x == current_pos {
            transform.translation.x = farest + SPACE_BETWEEN_PIPES;
        }
    }
}

fn despawn_pipes(mut commands: Commands, query: Query<Entity, With<Pipe>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
