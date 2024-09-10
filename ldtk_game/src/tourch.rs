use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::prelude::*;

pub struct TourchPlugin;

impl Plugin for TourchPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<TourchBundle>("Tourch")
            .add_systems(Update, setup);
    }
}

#[derive(Default, Component)]
struct Tourch;

#[derive(Default, Bundle, LdtkEntity)]
struct TourchBundle {
    tourch: Tourch,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn setup(mut commands: Commands, tourches: Query<Entity, Added<Tourch>>) {
    for tourch in &tourches {
        commands.entity(tourch).insert((
            PointLight2d {
                radius: 100.0,
                intensity: 3.0,
                ..default()
            },
            Collider::rectangle(16.0, 16.0),
            RigidBody::Static,
        ));
    }
}
