use bevy::{prelude::*, ecs::query::QueryEntityError};

// TODO: Add dino physic.
pub struct DinoPlugin;

impl Plugin for DinoPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<DinoState>()
            .add_systems(Startup, dino_setup)
            .add_systems(Update, dino_jump)
            .add_systems(Update, animate_sprite);
    }
}

#[derive(Component)]
struct Dino;

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
                sprite.index + 1
            };
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, States)]
pub enum DinoState {
    #[default]
    Idle,
    Running,
}

fn dino_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // TODO: Dino spreed sheet
    let texture_handle = asset_server.load("dino.png");
    let frame = 96.;
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(frame, frame), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 1 };
    commands.spawn((Dino, Name::new("Dino"), SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(animation_indices.first),
        transform: Transform {
            translation: Vec3 { x: -480., y: -250., z:  2.},
            ..default()
        },
        ..default()
    },
    animation_indices,
    AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating))));
}

fn dino_jump(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Dino>>) {
    if keys.pressed(KeyCode::Space) {
        let mut transform = query.single_mut();

        transform.translation.y += 30.;
    }
}
