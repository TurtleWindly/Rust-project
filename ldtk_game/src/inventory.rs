use crate::player::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_tourch);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum InventoryAction {
    Interact,
}

#[derive(Default, Bundle)]
pub struct InventoryBundle();

impl InventoryBundle {
    pub fn default_input_map() -> InputMap<InventoryAction> {
        use InventoryAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(Interact, MouseButton::Left);
        input_map.insert(Interact, KeyCode::KeyU);

        input_map
    }
}

fn spawn_tourch(
    inventory_action: Query<&ActionState<InventoryAction>, With<Player>>,
    window: Query<&Window, (With<PrimaryWindow>, Without<Player>)>,
) {
    for action_state in &inventory_action {
        // let bar = action_state.just_pressed(&InventoryAction::Interact);
        // println!("{}", bar);
        println!("{:?}", action_state);
        if action_state.pressed(&InventoryAction::Interact) {
            println!("pressed");
        }
    }
}
