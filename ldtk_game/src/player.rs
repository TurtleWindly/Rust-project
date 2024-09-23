use avian2d::math::*;
use avian2d::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    setup_player,
                    move_player_from_input,
                    update_camera,
                    translate_grid_coords_entities,
                )
                    .chain(),
            );
    }
}

const GRID_SIZE: i32 = 16;
const CAM_LERP_FACTOR: f32 = 2.0;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
}

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

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Right, KeyCode::KeyD);

        input_map
    }
}

fn setup_player(mut commands: Commands, players: Query<Entity, Added<Player>>) {
    for player in &players {
        commands.entity(player).insert((
            RigidBody::Dynamic,
            Collider::rectangle(16.0, 32.0),
            LockedAxes::ROTATION_LOCKED,
            InputManagerBundle::with_map(PlayerBundle::default_input_map()),
        ));
    }
}

fn move_player_from_input(
    // keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<
        (
            &MovementSpeed,
            &mut LinearVelocity,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
) {
    for (movement_speed, mut linear_velocity, action_state) in &mut players {
        let up = action_state.pressed(&PlayerAction::Up);
        let down = action_state.pressed(&PlayerAction::Down);
        let left = action_state.pressed(&PlayerAction::Left);
        let right = action_state.pressed(&PlayerAction::Right);
        let horizontal = right as i8 - left as i8;
        let vertical = up as i8 - down as i8;
        // TODO: WTF
        linear_velocity.x = horizontal as Scalar * movement_speed.0;
        linear_velocity.y = vertical as Scalar * movement_speed.0;
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut GridCoords, &Transform), Changed<Transform>>,
) {
    for (mut grid_coords, transform) in grid_coords_entities.iter_mut() {
        *grid_coords = bevy_ecs_ldtk::utils::translation_to_grid_coords(
            transform.translation.truncate(),
            IVec2::splat(GRID_SIZE),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera2d = Camera2dBundle::default();
    camera2d.projection.scale = 0.3;
    camera2d.camera.hdr = true;

    commands.spawn((camera2d, BloomSettings::NATURAL));
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation = camera
        .translation
        .lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR);
}
