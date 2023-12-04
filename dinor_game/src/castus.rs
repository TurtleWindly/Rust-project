use crate::dino::FrameSize;
use bevy::prelude::*;
use crate::GameState;

pub struct CastusPlugin;

impl Plugin for CastusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), castus_spawner);
    }
}

#[derive(Component)]
struct CastusSpawner;

#[derive(Component)]
struct Castus;

fn castus_spawner(mut commands: Commands) {
    // let texture = asset_server.load("castus.png");
    // let frame_size = (30., 74.);
    let spawn_pos = Vec2 { x: 700., y: -250. };
    commands.spawn((
        CastusSpawner,
        Name::new("Castus Spawner"),
        TransformBundle {
            local: Transform {
                translation: Vec3 {
                    x: spawn_pos.x,
                    y: spawn_pos.y,
                    z: 2.,
                },
                ..default()
            },
            ..default()
        },
    ));
}

// Castus spawner spawn castus
