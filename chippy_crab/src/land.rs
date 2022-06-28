use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LandPlugin;

impl Plugin for LandPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_land);
    }
}

fn spawn_land(mut commands: Commands, window_des: Res<WindowDescriptor>) {
    let width = window_des.width;
    let height = 64.;

    let sprite = Sprite {
        color: Color::rgb_u8(0, 255, 0),
        custom_size: Some(Vec2::new(width, height)),
        ..default()
    };

    commands
        .spawn_bundle(SpriteBundle {
            sprite,
            transform: Transform {
                translation: Vec3::new(window_des.width / 2., height / 2., 0.),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(width, height / 2.));
}
