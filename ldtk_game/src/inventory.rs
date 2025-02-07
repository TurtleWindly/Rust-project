use crate::player::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::PlayerAction;
use crate::tourch::{Lighter, DEFAULT_INTENSITY};
use crate::GRID_SIZE;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_tourch);
    }
}

#[derive(Default, Bundle)]
pub struct InventoryBundle();

fn spawn_tourch(
    player_qs: Query<(&ActionState<PlayerAction>, &Transform), With<Player>>,
    mut commands: Commands,
    // window: Query<&Window, (With<PrimaryWindow>, Without<Player>)>,
) {
    for (action_state, transform) in &player_qs {
        if action_state.just_pressed(&PlayerAction::SpawnTourch) {
            // Get location of player to spawn tourch
            let lighter_x = transform.translation.x.trunc() as i32 / GRID_SIZE;
            let lighter_y = transform.translation.y.trunc() as i32 / GRID_SIZE;
            println!("{}|{}", lighter_x, lighter_y);
            // Spawn lighter
            commands.spawn((
                Lighter,
                PointLight {
                    radius: 100.0,
                    intensity: DEFAULT_INTENSITY,
                    ..Default::default()
                },
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            (lighter_x * GRID_SIZE) as f32,
                            (lighter_y * GRID_SIZE) as f32,
                            100.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        }
    }
}
