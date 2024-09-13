use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const GRID_SIZE: Scalar = 16.0;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
}

fn setup(mut commands: Commands, wall_qs: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>) {
    for (entity, tile_enum) in &wall_qs {
        print!("{}: {}", entity, tile_enum.tags[0]);
        commands
            .entity(entity)
            .insert((Collider::rectangle(GRID_SIZE, GRID_SIZE), RigidBody::Static));
    }
}
