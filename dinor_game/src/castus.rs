use std::time::Duration;

use crate::dino::FrameSize;
use crate::GameState;
use bevy::prelude::*;

pub struct CastusPlugin;

impl Plugin for CastusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), castus_spawner)
            .add_systems(Update, (spawn_castus, castus_moving));
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
        VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        },
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
            let frame_size = (30., 74.);
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Castus,
                    Name::new("Castus"),
                    FrameSize(frame_size.0, frame_size.1),
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
