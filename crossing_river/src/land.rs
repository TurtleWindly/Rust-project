use bevy::prelude::*;
use crate::GameState;

pub struct LandPlugin;

impl Plugin for LandPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_land)
        ;
    }
}

fn spawn_land(mut commands: Commands, asset_server: Res<AssetServer>) {
    let land = asset_server.load("land.png");

    commands.spawn((SpriteBundle {
        texture: land.clone(),
        transform: Transform {
            translation: Vec3 { x: 670., y: -280., z: 2.},
            ..default()
        },
        ..default()
    }, Name::new("Right Land")));

    commands.spawn((SpriteBundle {
        texture: land.clone(),
        transform: Transform {
            translation: Vec3 { x: -700., y: -280., z: 2.},
            ..default()
        },
        sprite: Sprite {
          flip_x: true,
            ..default()
        },
        ..default()
    }, Name::new("Left Land")));
}