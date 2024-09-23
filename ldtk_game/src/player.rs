use avian2d::math::*;
use avian2d::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
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

fn setup_player(mut commands: Commands, players: Query<Entity, Added<Player>>) {
    for player in &players {
        commands.entity(player).insert((
            RigidBody::Dynamic,
            Collider::rectangle(16.0, 32.0),
            LockedAxes::ROTATION_LOCKED,
        ));
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
