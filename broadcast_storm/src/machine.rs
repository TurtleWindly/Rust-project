use bevy::{prelude::*, render::camera::RenderTarget};

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
        app.add_startup_system(spawn_machine)
        .add_system(my_cursor_system);
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
    // for index in 0..2 {
    //     commands.spawn_bundle(SpriteBundle {
    //         sprite: pc_sprite.clone(),
    //         transform: Transform {
    //             translation: Vec3::new( index as f32 * 300. + 300., index as f32 * 200. + 100., 0.),
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(PC)
    //     .insert(Machine{ id: format!("PC{}", index)});
    // }

    // // Spawn Switch
    // // 1
    // commands.spawn_bundle(SpriteBundle {
    //     sprite: sw_sprite.clone(),
    //     transform: Transform {
    //         translation: Vec3::new( 100., 200., 0.),
    //         ..default()
    //     },
    //     ..default()
    // })
    // .insert(Switch)
    // .insert(Machine{ id: "Switch1".into()});
    // // 2
    // commands.spawn_bundle(SpriteBundle {
    //     sprite: sw_sprite.clone(),
    //     transform: Transform {
    //         translation: Vec3::new( 100., 400., 0.),
    //         ..default()
    //     },
    //     ..default()
    // })
    // .insert(Switch)
    // .insert(Machine{ id: "Switch2".into()});
    // // 3
    // commands.spawn_bundle(SpriteBundle {
    //     sprite: sw_sprite.clone(),
    //     transform: Transform {
    //         translation: Vec3::new( 350., 300., 0.),
    //         ..default()
    //     },
    //     ..default()
    // })
    // .insert(Switch)
    // .insert(Machine{ id: "Switch3".into()});
}

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
    // Check mouse event
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
) {
    if ! buttons.just_pressed(MouseButton::Left) {
        return;
    }

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

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let mut world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        world_pos.z = 1.;
        println!("World coords: {}/{}", world_pos.x, world_pos.y);

        commands.spawn_bundle(SpriteBundle {
            sprite: sw_sprite.clone(),
            transform: Transform {
                translation: world_pos,
                ..default()
            },
            ..default()
        })
        .insert(Switch);
        // .insert(Machine{ id: "Switch3".into()});
    }
}