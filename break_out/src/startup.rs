use bevy::prelude::*;
use crate::component::*;

pub struct StartupPlugin;

struct BlockSpawnDetail {
    gaps: f32,
    column: i32,
    row: i32,
    width: f32,
    height: f32,
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                title: "debug".to_string(),
                width: 900.,
                height: 700.,
                ..default()
            })
            .insert_resource(BlockSpawnDetail {
                gaps: 10.,
                column: 5,
                row: 3,
                width: 100.,
                height: 20.,
            })
            .add_startup_system(startup_spawn)
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

fn startup_spawn(
    mut commands: Commands,
    brick_detail: Res<BlockSpawnDetail>,
    window: Res<WindowDescriptor>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn Player
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(255, 255, 255),
            custom_size: Some(Vec2::new(100., 20.)),
            ..default()
        },
        transform: Transform::from_xyz(0., -250., 0.),
        ..default()
    }).insert(Player);

    // Wall spawn section
    let wall_thickness = 100.;

    // UpWall
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(128, 128, 128),
            // 100 is debug size
            custom_size: Some(Vec2::new(window.width, wall_thickness)),
            ..default()
        },
        transform: Transform::from_xyz(0., window.height / 2., 1.),
        ..default()
    }).insert(Wall);

    //DownWall
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(128, 128, 128),
            // 100 is debug size
            custom_size: Some(Vec2::new(window.width, wall_thickness)),
            ..default()
        },
        transform: Transform::from_xyz(0., window.height / 2. * -1., 1.),
        ..default()
    }).insert(Wall);

    // Brick spawn section
    let offset_x:f32 = -100.;
    let offset_y:f32 = 100.;

    for row in 0..brick_detail.row {
        for column in 0..brick_detail.column {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb_u8(0, 0, 0),
                        custom_size: Some(Vec2::new(brick_detail.width, brick_detail.height)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                                        offset_x + column as f32 * (brick_detail.width + brick_detail.gaps),
                                        offset_y + row as f32 * (brick_detail.height + brick_detail.gaps),
                                        0.),
                        ..default()
                    },
                    ..default()
                })
            .insert(Block);
        }
    }

}
