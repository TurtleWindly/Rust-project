use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use crate::{ascii::{AsciiSheet, spawn_ascii_sprite}, TITLE_SIZE};
use crate::titlemap::TitleCollider;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement.label("player movement"))
        .add_system(camera_follow_player.after("player movement"))
        ;
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player =  spawn_ascii_sprite(&mut commands, &ascii, 1, Color::rgb_u8(255, 255, 255), Vec3::new(2. * TITLE_SIZE, 2. *TITLE_SIZE, 111.));

    commands.entity(player).insert(Player { speed: 5. }).insert(Name::new("Player"));

    let background_player = spawn_ascii_sprite(&mut commands, &ascii, 0, Color::rgb(0., 0., 0.), Vec3::new(0., 0., 0.));
    commands.entity(background_player).insert(Name::new("Player_Background"));

    commands.entity(player).push_children(&[background_player]);
}

fn player_movement(
    mut query: Query<(&Player ,&mut Transform)>,
    wall_query: Query<&Transform, (With<TitleCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut tranform) = query.single_mut();

    let mut pre_x = 0.;
    let mut pre_y = 0.;

    if keyboard.pressed(KeyCode::W) {
        pre_y += player.speed * TITLE_SIZE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::S) {
        pre_y -= player.speed * TITLE_SIZE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::A) {
        pre_x -= player.speed * TITLE_SIZE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::D) {
        pre_x += player.speed * TITLE_SIZE * time.delta_seconds();
    }

    let target = tranform.translation + Vec3::new(pre_x, 0., 0.);
    if check_wall_collision(target, &wall_query) {
        tranform.translation = target;
    }

    let target = tranform.translation + Vec3::new(0., pre_y, 0.);
    if check_wall_collision(target, &wall_query) {
        tranform.translation = target;
    }

}

fn check_wall_collision(
    player_position: Vec3,
    wall_query: &Query<&Transform, (With<TitleCollider>, Without<Player>)>,
) -> bool {
    for wall in wall_query.iter() {
        let collision = collide(
            player_position,
            Vec2::splat(TITLE_SIZE * 0.9),
            wall.translation,
            Vec2::splat(TITLE_SIZE),
            );

        if collision.is_some() {
            return false;
        }
    }
    true
}

fn camera_follow_player(
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let player = player.single();
    let mut camera = camera.single_mut();

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
