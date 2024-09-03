use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<GoalBundle>("Goal")
            .add_systems(Update, check_goal);
    }
}

#[derive(Default, Component)]
struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    goal: Goal,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn check_goal(
    level_selection: ResMut<LevelSelection>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    goals: Query<&GridCoords, With<Goal>>,
) {
    if players
        .iter()
        .zip(goals.iter())
        .any(|(player_grid_coords, goal_grid_coords)| player_grid_coords == goal_grid_coords)
    {
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("Level go brrr"),
        };
        indices.level += 1;
    }
}
