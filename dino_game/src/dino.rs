use bevy::prelude::*;
use std::time::Duration;

use crate::GameState;

pub struct DinoPlugin;

impl Plugin for DinoPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<DinoState>()
            .add_systems(Startup, dino_setup)
            .add_systems(OnEnter(GameState::Game), (to_running_state, running_dino_sprite))
            .add_systems(OnEnter(DinoState::Collided), collided_dino_sprite)
            .add_systems(Update, dino_jump.run_if(in_state(DinoState::Running)))
            .add_systems(
                Update,
                handle_jump
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(DinoState::Jumping)),
            )
            .add_systems(Update, animate_sprite)
            .add_systems(OnExit(GameState::ScoreMenu), reset_dino);
    }
}

#[derive(Component)]
pub struct Dino;

#[derive(Component)]
struct JumpTime {
    timer: Timer,
}

#[derive(Component)]
pub struct StartingPosition(Vec2);

#[derive(Component)]
pub struct FrameSize {
    pub vector: Vec2,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                if sprite.index > indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                }
            };
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, States)]
pub enum DinoState {
    #[default]
    Idle,
    Collided,
    Running,
    Jumping,
    Falling,
}

fn dino_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dino.png");
    let frame_size = Vec2::new(88., 92.);
    let starting_pos = Vec2 { x: -480., y: -250. };
    let texture_atlas = TextureAtlas::from_grid(texture_handle, frame_size, 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 1 };
    commands.spawn((
        Dino,
        FrameSize { vector: frame_size },
        Name::new("Dino"),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3 {
                    x: starting_pos.x,
                    y: starting_pos.y,
                    z: 3.,
                },
                ..default()
            },
            ..default()
        },
        JumpTime {
            timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Once),
        },
        StartingPosition(starting_pos),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));
}

fn dino_jump(keys: Res<Input<KeyCode>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Space) {
        commands.insert_resource(NextState(Some(DinoState::Jumping)));
    }
}

fn handle_jump(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut JumpTime, &mut Transform, &StartingPosition), With<Dino>>,
) {
    let jump_speed = 700.;

    if let Ok((mut jump, mut transform, starting_pos)) = query.get_single_mut() {
        jump.timer.tick(time.delta());

        transform.translation.y = min(
            transform.translation.y
                + jump_speed
                    * time.delta_seconds()
                    * (jump.timer.remaining_secs() - jump.timer.elapsed_secs()),
            starting_pos.0.y,
        );

        if transform.translation.y == starting_pos.0.y {
            jump.timer.reset();
            commands.insert_resource(NextState(Some(DinoState::Running)));
        }
    }
}

fn min(value: f32, min: f32) -> f32 {
    if value < min {
        return min;
    }
    value
}

fn reset_dino(
    mut commands: Commands,
    mut dino: Query<(&mut Transform, &StartingPosition), With<Dino>>,
) {
    commands.insert_resource(NextState(Some(DinoState::Running)));

    // Reset Dino Position
    if let Ok((mut transform, starting_pos)) = dino.get_single_mut() {
        transform.translation = starting_pos.0.extend(2.);
    }
}

fn to_running_state(mut commands: Commands) {
    commands.insert_resource(NextState(Some(DinoState::Running)));
}

fn running_dino_sprite(mut query: Query<(&mut AnimationIndices, &mut AnimationTimer), With<Dino>>) {
    if let Ok((mut animation_indices, mut timer)) = query.get_single_mut() {
        timer.0 = Timer::new(Duration::from_millis(100), TimerMode::Repeating);
        animation_indices.first = 2;
        animation_indices.last = 3;
    }
}

// Adding when dino collide with castus change sprite to dead dino
fn collided_dino_sprite(mut query: Query<(&mut AnimationIndices, &mut TextureAtlasSprite), With<Dino>>) {
    for (mut animation_indices, mut sprite) in query.iter_mut() {
        animation_indices.first = 4;
        animation_indices.last = 4;
        // Dead Dino Sprite index
        sprite.index = 4;
    }
}
