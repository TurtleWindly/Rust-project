use bevy::prelude::*;

#[derive(Component)]
struct Machine {
    id: String,
}


#[derive(Component)]
struct PC;

#[derive(Component)]
struct Switch;

pub struct MachinePlugin;

impl Plugin for MachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_machine);
    }
}

fn spawn_machine(mut commands: Commands) {
    let pc_sprite = Sprite {
        color: Color::rgb_u8(10, 10, 255),
        custom_size: Some(Vec2::new( 100., 100.)),
        ..default()
    };

    let sw_sprite = Sprite {
        color: Color::rgb_u8(10, 255, 10),
        custom_size: Some(Vec2::new( 140., 60.)),
        ..default()
    };

    // Spawn PC
    for index in 0..2 {
        commands.spawn_bundle(SpriteBundle {
            sprite: pc_sprite.clone(),
            transform: Transform {
                translation: Vec3::new( index as f32 * 300. + 300., index as f32 * 200. + 100., 0.),
                ..default()
            },
            ..default()
        });
    }

    // Spawn Switch
    // 1
    commands.spawn_bundle(SpriteBundle {
        sprite: sw_sprite.clone(),
        transform: Transform {
            translation: Vec3::new( 100., 200., 0.),
            ..default()
        },
        ..default()
    });
    // 2
    commands.spawn_bundle(SpriteBundle {
        sprite: sw_sprite.clone(),
        transform: Transform {
            translation: Vec3::new( 100., 400., 0.),
            ..default()
        },
        ..default()
    });
    // 3
    commands.spawn_bundle(SpriteBundle {
        sprite: sw_sprite.clone(),
        transform: Transform {
            translation: Vec3::new( 350., 300., 0.),
            ..default()
        },
        ..default()
    });
}