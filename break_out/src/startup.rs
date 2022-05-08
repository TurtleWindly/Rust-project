use bevy::prelude::*;
use crate::component::*;

pub struct StartupPlugin;

struct BlockSpawnDetail {
    gaps: i32,
    column: i32,
    row: i32,
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                title: "Break out!".to_string(),
                width: 900.,
                height: 700.,
                ..default()
            })
            .insert_resource(BlockSpawnDetail {
                gaps: 10,
                column: 5,
                row: 3,
            })
            .add_startup_system(startup_spawn)
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

fn startup_spawn(
    mut commands: Commands,
    block_spawn_detail: Res<BlockSpawnDetail>,
    windows: Res<WindowDescriptor>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(255, 255, 255),
            custom_size: Some(Vec2::new(50., 20.)),
            ..default()
        },
        transform: Transform::from_xyz(0., -250., 0.),
        ..default()
    }).insert(Player);

    println!("{}", windows.width);

    for _row in 0..block_spawn_detail.row {
        for column in 0..block_spawn_detail.column {
            commands.spawn().insert(Block {
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb_u8(0, 0, 0),
                        custom_size: Some(Vec2::new((windows.width - (block_spawn_detail.gaps as f32 / 2.)) / block_spawn_detail.column as f32, 50.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                                   block_spawn_detail.gaps as f32 + (column as f32 * windows.width - (block_spawn_detail.gaps as f32 / 2.))
                                   , 0., 90.),
                    ..default()
                }
            });
        }
    }

}
