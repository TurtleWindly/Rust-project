use bevy::prelude::*;

#[derive(Default)]
struct SortingArray {
    value: Vec<i32>,
}

#[derive(Default)]
struct BarInfo {
    width: f32,
    height_in_screen: f32,
}

#[derive(Component)]
struct Bar;

pub struct SortingPlugin;

impl Plugin for SortingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SortingArray {
            value: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        })
        .insert_resource(BarInfo {
            width: 0.,
            height_in_screen: 0.,
        })
        .add_startup_system(calculate_bar_size.label("calculate"))
        .add_startup_system(draw_bar.after("calculate"))
        .add_system(sorting);
    }
}

fn calculate_bar_size(mut bar_info: ResMut<BarInfo>, array: Res<SortingArray>, windows_des: Res<WindowDescriptor>) {
    bar_info.width = windows_des.width / array.value.len() as f32;
    bar_info.height_in_screen = windows_des.height / array.value.len() as f32;
}

fn sorting(array: Res<SortingArray>) {
    for _a in &array.value {
    }
}

// Spawn the bar to correct location
fn draw_bar(array: Res<SortingArray>, bar_info: Res<BarInfo>, mut commands: Commands) {

    let mut index = 0f32;
    for element in &array.value {
        let width = bar_info.width;
        let height = bar_info.height_in_screen * element.clone() as f32;

        let sprite = Sprite {
            color: Color::rgb_u8(255, 255, 255),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        };

        commands.spawn_bundle(SpriteBundle {
            sprite,
            transform: Transform {
                translation: Vec3::new( width * index + width / 2., height / 2., 0.),
            ..default()},
            ..default()
        })
        .insert(Bar);
    
        index = index + 1.;
    }
}