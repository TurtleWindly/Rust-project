use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub enum Wall {
    Horizontal,
    Vertical,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Collider;
