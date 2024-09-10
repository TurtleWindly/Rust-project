use crate::player::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<ShopBundle>("Shop")
            .add_systems(Update, check_shop);
    }
}

#[derive(Default, Component)]
struct Shop;

#[derive(Default, Bundle, LdtkEntity)]
struct ShopBundle {
    shop: Shop,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn check_shop(
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    shop: Query<&GridCoords, With<Shop>>,
    mut exit: EventWriter<AppExit>,
) {
    if players
        .iter()
        .zip(shop.iter())
        .any(|(player_grid_coords, shop_grid_coords)| player_grid_coords == shop_grid_coords)
    {
        exit.send(AppExit::Success);
    }
}
