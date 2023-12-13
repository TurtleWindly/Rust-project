use std::time::Duration;

use crate::dino::{Dino, DinoState, FrameSize};
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub struct CastusPlugin;

impl Plugin for CastusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, castus_spawner)
            .add_systems(
                Update,
                (spawn_castus, castus_moving, castus_collision).run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::ScoreMenu), despawn_castus);
    }
}

#[derive(Component)]
struct CastusSpawner {
    timer: Timer,
}

#[derive(Component)]
struct Castus;

fn castus_spawner(mut commands: Commands) {
    // let texture = asset_server.load("castus.png");
    // let frame_size = (30., 74.);
    let spawn_pos = Vec2 { x: 700., y: -250. };
    commands.spawn((
        CastusSpawner {
            timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
        },
        Name::new("Castus Spawner"),
        SpriteBundle {
            transform: Transform {
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
fn spawn_castus(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CastusSpawner)>,
) {
    if let Ok((entity, mut spawner)) = query.get_single_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.finished() {
            let texture = asset_server.load("castus.png");
            let frame_size = Vec2::new(30., 74.);
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Castus,
                    Name::new("Castus"),
                    FrameSize { vector: frame_size },
                    SpriteBundle {
                        texture,
                        ..default()
                    },
                ));
            });
        }
    }
}

fn castus_moving(mut commands: Commands, mut query: Query<(&mut Transform, Entity), With<Castus>>) {
    for (mut castus, entity) in query.iter_mut() {
        castus.translation.x -= 10.;
        // -640 is windows size
        if castus.translation.x < -1350. {
            commands.entity(entity).despawn();
        }
    }
}

fn castus_collision(
    mut commands: Commands,
    castus_query: Query<(&GlobalTransform, &FrameSize), With<Castus>>,
    player: Query<(&Transform, &FrameSize), (With<Dino>, Without<Castus>)>,
) {
    if let Ok((player_transform, player_size)) = player.get_single() {
        let player_size_slimmer = Vec2::new(player_size.vector.x * 0.6, player_size.vector.y);
        for (castus_transform, castus_size) in castus_query.iter() {
            if collide(
                player_transform.translation,
                player_size_slimmer,
                castus_transform.translation(),
                castus_size.vector,
            )
            .is_some()
            {
                commands.insert_resource(NextState(Some(GameState::ScoreMenu)));
                commands.insert_resource(NextState(Some(DinoState::Collided)));
            }
        }
    }
}

fn despawn_castus(mut commands: Commands, castus_query: Query<Entity, With<Castus>>) {
    for entity in castus_query.iter() {
        commands.entity(entity).despawn();
    }
}
