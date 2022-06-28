use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_crab).add_system(jump);
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
                scale: Vec3::new(3., 3., 0.),
                translation: Vec3::new(100., window_des.height / 2., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(16., 16.))
        .insert(Velocity {
            linvel: Vec2::ZERO,
            ..default()
        })
        .insert(ColliderMassProperties::Density(2.0));
}

fn jump(
    mut query: Query<&mut Velocity, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    ) {
    if keyboard.just_pressed(KeyCode::Space) {
        let mut velocity = query.single_mut();

        velocity.linvel = Vec2::new(0., 175.);
    }

}
