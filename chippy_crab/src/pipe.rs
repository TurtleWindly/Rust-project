use bevy::prelude::*;
use rand::prelude::*;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pipes).add_system(pipes_move);
    }
}

#[derive(Component)]
pub struct Pipe;

// Spawn first time position pipes
fn spawn_pipes(mut commands: Commands, window_des: Res<WindowDescriptor>) {

    // Pairs of pipes
    let pipes_will_spawn: usize = 4;
    let pipe_width: f32 = 80.;
    let space_between_pipes: f32 = 200.;
    let free_space: f32 = 200.;

    let mut pipe_list: Vec<SpriteBundle> = Default::default();

    let mut rng = thread_rng();

    // Bot
    for index in 0..pipes_will_spawn {
        let random_heigth = rng.gen_range(80.0..(window_des.height - free_space - 100.));

        pipe_list.push(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(pipe_width, random_heigth)),
                color: Color::DARK_GREEN,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(window_des.width + (index as f32 * space_between_pipes), random_heigth / 2., 1.),
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(pipe_width, pipe_top_height)),
                color: Color::BLACK,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(window_des.width + (index as f32 * space_between_pipes), pipe_bot_height + free_space + (pipe_top_height / 2.), 1.),
                ..default()
            },
            ..default()
        });
    }

    for pipe in pipe_list {
        commands.spawn_bundle(pipe).insert(Pipe);
    }
}

fn pipes_move(mut query: Query<&mut Transform, With<Pipe>>) {
    for mut tranform in query.iter_mut() {
        tranform.translation.x -= 1.;
    }
}
