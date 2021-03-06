use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_crab))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(jump)
                    .with_system(jump_but_touch)
                    .with_system(jump_but_click)
                    .with_system(lose),
            );
    }
}

#[derive(Component)]
struct Player;

fn spawn_crab(mut commands: Commands, assets: Res<AssetServer>, window_des: Res<WindowDescriptor>) {
    let crab: Handle<Image> = assets.load("Crab.png");

    commands
        .spawn_bundle(SpriteBundle {
            texture: crab,
            transform: Transform {
                scale: Vec3::new(2.5, 2.5, 0.),
                translation: Vec3::new(150., window_des.height / 2., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(16., 12.))
        .insert(Velocity {
            linvel: Vec2::ZERO,
            ..default()
        })
        .insert(ColliderMassProperties::Density(2.0));
}

fn jump(mut query: Query<&mut Velocity, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        let mut velocity = query.single_mut();

        velocity.linvel = Vec2::new(0., 175.);
    }
}

fn jump_but_touch(mut query: Query<&mut Velocity, With<Player>>, touches: Res<Touches>) {
    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            let mut velocity = query.single_mut();
            velocity.linvel = Vec2::new(0., 175.);
        }
    }
}

fn jump_but_click(mut query: Query<&mut Velocity, With<Player>>, mouse: Res<Input<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut velocity = query.single_mut();
        velocity.linvel = Vec2::new(0., 175.);
    }
}

fn lose(
    query: Query<(&Transform, Entity), With<Player>>,
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
) {
    for (transform, entity) in query.iter() {
        if transform.translation.x < 0. {
            commands.entity(entity).despawn();
            game_state
                .set(GameState::ScoreMenu)
                .expect("Can't change game state when player lose");
        }
    }
}
