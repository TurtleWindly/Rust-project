use bevy::prelude::*;
use crate::component::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(generate_world)
            .set_runner(game_loop);
    }
}

fn generate_world(mut command: Commands) {
    command.spawn_bundle(PlayerBundle {
        location: Location("Tarven".to_string()),
        ..default()
    });

    command.spawn_bundle(StructureBundle {
        location: Location("Tarven".to_string()),
            ..default()
    });
    command.spawn_bundle(StructureBundle {
        location: Location("Dungeon".to_string()),
            ..default()
    });
}

fn game_loop(mut app: App) {
    loop {
        app.update();
    }
}
