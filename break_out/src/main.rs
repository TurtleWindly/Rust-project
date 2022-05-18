use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

const TIME_STEP: f32 = 1. / 60.;

use crate::startup::*;
use crate::component::*;

mod startup;
mod component;

fn main() {
    App::new()
        .add_plugin(StartupPlugin)
        .add_plugins(DefaultPlugins)
        .add_system(player_movement)
        .add_system(apply_velocity)
        .add_system(check_for_collision)
        .run();
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
        for mut tranform in query.iter_mut() {
            tranform.translation.x -= 10.;
        }
    }

    if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
        for mut tranform in query.iter_mut() {
            tranform.translation.x += 10.;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn check_for_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Block>), With<Collider>>
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for (collider_entity, transform, maybe_this_is_block) in collider_query.iter() {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
           );

        println!("{:?}", collision);

        if let Some(collision) = collision {
            // despawn Block
            if maybe_this_is_block.is_some() {
                commands.entity(collider_entity).despawn();
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}
