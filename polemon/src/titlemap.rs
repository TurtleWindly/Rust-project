use bevy::prelude::*;
use std::{fs::File, io::BufReader, io::BufRead};

use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::TITLE_SIZE;

pub struct TitleMapPlugin;

#[derive(Component)]
pub struct TitleCollider;

impl Plugin for TitleMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(creat_map);
    }
}

fn creat_map(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
) {
    let file = File::open("../assets/map.txt").expect("No file found");
    let mut titles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let title = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(1., 1., 1.),
                    Vec3::new(x as f32 * TITLE_SIZE, (y as f32) * TITLE_SIZE, 100.,),
                    );

                if char == '#' {
                    commands.entity(title).insert(TitleCollider);
                }

                titles.push(title);
            }
        }
    }
    commands.spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&titles);
}
