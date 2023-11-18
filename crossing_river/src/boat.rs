use bevy::prelude::*;
use crate::GameState;

pub struct BoatPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, States)]
pub enum BoatState {
    #[default]
    Standing,
    Moving,
}

impl Plugin for BoatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<BoatState>()
            .add_systems(OnEnter(GameState::Game), (spawn_boat, spawn_button_move_boat))
            .add_systems(Update, check_button_boat.run_if(in_state(GameState::Game)))
            .add_systems(Update, move_boat.run_if(in_state(BoatState::Moving)))
        ;
    }
}

#[derive(Component)]
struct ButtonMoveBoat;

#[derive(Component)]
struct Boat;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
}

fn spawn_boat(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("boat.png");

    commands.spawn((SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3 { x: 140., y: -260., z: 1.},
            ..default()
        },
        ..default()
    }, Boat, Direction::Left, Name::new("Boat")));
}

fn spawn_button_move_boat(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((ButtonMoveBoat, Name::new("Button Move Boat"), ButtonBundle {
        style: Style {
            width: Val::Px(150.),
            height: Val::Px(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb_u8(255, 255, 255).into(),
        ..default()
    })).with_children(|parent| {
            parent.spawn((Name::new("Text Button"), TextBundle::from_section("Move", TextStyle {
                font: asset_server.load("fonts/HackNerdFont.ttf"),
                font_size: 25.,
                color: Color::BLACK,
            },
            ).with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                    position_type: PositionType::Relative,
                    ..default()
            })));
        });
}

fn check_button_boat(
    mut commands: Commands,
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>, With<ButtonMoveBoat>, Without<Boat>
        )
    >
) {
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState(Some(BoatState::Moving)));
            }
            _ => {}
        }
    }
}

fn move_boat(mut commands: Commands, time: Res<Time>, mut boat_query: Query<(&mut Transform, &mut Direction), With<Boat>>) {
    let speed = 100.;
    let max_distance = 140.;
    
    if let Ok((mut transform, mut direction)) = boat_query.get_single_mut() {
        match *direction {
            Direction::Left => transform.translation.x -= speed * time.delta_seconds(),
            Direction::Right => transform.translation.x += speed * time.delta_seconds(),
        }

        if transform.translation.x > max_distance {
            *direction = Direction::Left;
            commands.insert_resource(NextState(Some(BoatState::Standing)));
        } else if transform.translation.x < -max_distance {
            *direction = Direction::Right;
            commands.insert_resource(NextState(Some(BoatState::Standing)));
        }
    }
}
