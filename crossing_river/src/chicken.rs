use bevy::prelude::*;
use crate::boat::Direction;
use crate::{GameState, ObjectLocation, Object};

pub struct ChickenPlugin;

impl Plugin for ChickenPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_chicken)
        ;
    }
}

#[derive(Component)]
struct Chicken;

fn spawn_chicken(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("chicken.png");
    let chicken_size: f32 = 100.;

    commands.spawn((Chicken, Object, ObjectLocation::Land, Direction::Left, Name::new("Chicken"), ButtonBundle {
        style: Style {
            width: Val::Px(chicken_size),
            height: Val::Px(chicken_size),
            ..default()
        },
        image: UiImage {
            texture,
            ..default()   
        },
        ..default()
    }));
}
