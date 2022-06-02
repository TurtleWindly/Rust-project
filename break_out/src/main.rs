use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

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
        .add_system(check_for_collision_wall)
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

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_for_collision_wall(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    wall_query: Query<(&Transform, &Wall), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    let target = ball_transform.translation + Vec3::new(ball_velocity.x, 0., 0.);
    for (wall_tranform, wall) in wall_query.iter() {
        let collision = collide(target, ball_size, wall_tranform.translation, wall_tranform.translation.truncate());

        if collision.is_some() {
            match wall {
                Wall::Horizontal => {
                    ball_velocity.y = -ball_velocity.y;
                }
                Wall::Vertical => {
                    ball_velocity.x = -ball_velocity.x;
                }
            }
        }
    }

}
