use avian2d::math::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (move_player_from_input, setup));
    }
}

const GRID_SIZE: i32 = 16;

#[derive(Default, Component)]
pub struct Player;

#[derive(Clone, Component, Debug, Default, PartialEq)]
pub struct MovementSpeed(f32);

impl From<&EntityInstance> for MovementSpeed {
    fn from(entity_instance: &EntityInstance) -> Self {
        if let Ok(movement_speed) = entity_instance.get_float_field("MovementSpeed") {
            return MovementSpeed(*movement_speed);
        } else {
            panic!("Fuck");
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    movement_speed: MovementSpeed,
}

fn setup(mut commands: Commands, players: Query<Entity, Added<Player>>) {
    for player in &players {
        commands
            .entity(player)
            .insert((RigidBody::Kinematic, Collider::rectangle(16.0, 32.0)));
    }
}

fn move_player_from_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&MovementSpeed, &mut LinearVelocity), With<Player>>,
) {
    for (movement_speed, mut linear_velocity) in &mut players {
        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        let horizontal = right as i8 - left as i8;
        let vertical = up as i8 - down as i8;
        // TODO: WTF
        linear_velocity.x = horizontal as Scalar * movement_speed.0;
        linear_velocity.y = vertical as Scalar * movement_speed.0;
    }
    // } else if input.just_pressed(KeyCode::KeyA) {
    //     GridCoords::new(-1, 0)

    // for (mut player_grid_coords, linear_velocity) in players.iter_mut() {
    //     let destination = *player_grid_coords + movement_direction;
    //     *player_grid_coords = destination;
    // }
}

// fn translate_grid_coords_entities(
//     mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
// ) {
//     for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
//         transform.translation =
//             bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
//                 .extend(transform.translation.z);
//     }
// }
