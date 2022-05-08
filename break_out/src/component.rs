use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Block {
    pub sprite: SpriteBundle,
}
