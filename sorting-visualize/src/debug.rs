use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(moving_camera);
    }
}

fn moving_camera(mut camera: Query<&mut Transform, With<Camera>>, keys: Res<Input<KeyCode>>) {
    let mut camera = camera.single_mut();

    if keys.pressed(KeyCode::A) {
        camera.translation.x -= 5.;
    }

    if keys.pressed(KeyCode::D) {
        camera.translation.x += 5.;
    }

    if keys.pressed(KeyCode::S) {
        camera.translation.y -= 5.;
    }

    if keys.pressed(KeyCode::W) {
        camera.translation.y += 5.;
    }
}
