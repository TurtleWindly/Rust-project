use crate::player::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::PlayerAction;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_tourch);
    }
}

#[derive(Default, Bundle)]
pub struct InventoryBundle();

fn spawn_tourch(
    inventory_action: Query<&ActionState<PlayerAction>, With<Player>>,
    window: Query<&Window, (With<PrimaryWindow>, Without<Player>)>,
) {
    for action_state in &inventory_action {
        if action_state.pressed(&PlayerAction::SpawnTourch) {
            println!("pressed");
            println!("aaa");
            println!("bbbb");
            println!("bbbb");
            println!("bbbb");
        }
    }
}
