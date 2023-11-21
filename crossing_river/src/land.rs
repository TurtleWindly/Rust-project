use crate::GameState;
use bevy::prelude::*;

pub struct LandPlugin;

impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (spawn_land, setup_slot));
    }
}

#[derive(Component)]
enum SlotLocation {
    Left,
    Right,
}

#[derive(Component)]
struct Slot {
    empty: bool,
}

fn spawn_land(mut commands: Commands, asset_server: Res<AssetServer>) {
    let land = asset_server.load("land.png");

    commands.spawn((
        SpriteBundle {
            texture: land.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 670.,
                    y: -280.,
                    z: 2.,
                },
                ..default()
            },
            ..default()
        },
        Name::new("Right Land"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: land.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: -700.,
                    y: -280.,
                    z: 2.,
                },
                ..default()
            },
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..default()
        },
        Name::new("Left Land"),
    ));
}

fn setup_slot(mut commands: Commands) {
    let number_of_slot = 3;
    let slot_custom_size = Vec2 { x: 100., y: 100. };
    let land_left = 150.;
    let land_right = -150.;
    let slot_height = -250.;

    commands
        .spawn((
            SlotLocation::Left,
            Name::new("Slot Group Left"),
            SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: land_left,
                        y: slot_height,
                        z: 3.,
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            for slot in 1..=number_of_slot {
                parent.spawn((
                    Slot { empty: false },
                    Name::new("Slot"),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::Rgba {
                                red: 0.,
                                green: 0.,
                                blue: 0.,
                                alpha: 0.5,
                            },
                            custom_size: Some(slot_custom_size),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3 {
                                x: land_left * slot as f32,
                                y: 0.,
                                z: 3.,
                            },
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        });

    commands
        .spawn((
            SlotLocation::Right,
            Name::new("Slot Group Right"),
            SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: land_right,
                        y: slot_height,
                        z: 3.,
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            for slot in 1..=number_of_slot {
                parent.spawn((
                    Slot { empty: false },
                    Name::new("Slot"),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::Rgba {
                                red: 0.,
                                green: 0.,
                                blue: 0.,
                                alpha: 0.5,
                            },
                            custom_size: Some(slot_custom_size),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3 {
                                x: land_right * slot as f32,
                                y: 0.,
                                z: 3.,
                            },
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        });
}
