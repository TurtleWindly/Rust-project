use crate::{GameState, Object};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LandSetupSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnObjectSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AfterSpawnObjectSet;

#[derive(Component)]
enum SlotLocation {
    Left,
    Right,
}

#[derive(Component)]
struct Slot {
    empty: bool,
}

pub struct LandPlugin;

impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(
            Update,
            (LandSetupSet.before(SpawnObjectSet), SpawnObjectSet.before(AfterSpawnObjectSet)),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (
                spawn_land.before(setup_slot).in_set(LandSetupSet),
                setup_slot.before(relocate_object_to_slot).in_set(LandSetupSet),
                relocate_object_to_slot.in_set(AfterSpawnObjectSet),
            ),
        );
    }
}

fn spawn_land(mut commands: Commands, asset_server: Res<AssetServer>) {
    let land = asset_server.load("land.png");
    println!("1");

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

    println!("2");

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
                    Slot { empty: true },
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
                    Slot { empty: true },
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

fn relocate_object_to_slot(
    mut object_query: Query<(&mut Transform, &mut Object), With<Object>>,
    mut slot_query: Query<(&Transform, &mut Slot), (With<Slot>, Without<Object>)>,
) {
    for (mut object_transform, mut object) in object_query.iter_mut() {
        println!("{:?}", object_transform);
        for (slot_transform, mut slot) in slot_query.iter_mut() {
            if slot.empty {
                println!("succes");
                object_transform.translation = slot_transform.translation;
                object.located = true;
                slot.empty = false;
                break;
            }
        }
    }
    println!("4");
    for (mut transform, mut object) in object_query.iter_mut() {
        println!("{}", object.located);
    }
}
