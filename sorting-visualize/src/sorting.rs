use bevy::prelude::*;

use crate::array;

#[derive(Default)]
pub struct SortingArray {
    pub value: Vec<i32>,
    pub loop_times: i32,
    pub sorted_times: i32,
    // Check if arr need swapping, if not is sorting will false meaning the arr is done
    pub is_swapping: bool,
    pub is_sorting: bool,
}

#[derive(Default)]
struct BarInfo {
    width: f32,
    height_in_screen: f32,
}

#[derive(Component)]
struct Bar(i32);

pub struct SortingPlugin;

impl Plugin for SortingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SortingArray {
            // Create an array then shuffle it
            value: array::shuffle_vec( array::create_vec(0, 100)),
            loop_times: 0,
            sorted_times: 0,
            is_swapping: false,
            is_sorting: true,
        })
        .insert_resource(BarInfo {
            width: 0.,
            height_in_screen: 0.,
        })
        .add_startup_system(calculate_bar_size.label("calculate"))
        .add_startup_system(draw_bar.after("calculate"))
        .add_system(update_bar.label("update"));
    }
}

fn calculate_bar_size(
    mut bar_info: ResMut<BarInfo>,
    array: Res<SortingArray>,
    windows_des: Res<WindowDescriptor>,
) {
    bar_info.width = windows_des.width / array.value.len() as f32;
    bar_info.height_in_screen = windows_des.height / array.value.len() as f32;
}

// Spawn the bar to correct location
fn draw_bar(array: Res<SortingArray>, bar_info: Res<BarInfo>, mut commands: Commands) {
    let mut index = 0f32;
    for element in &array.value {
        let width = bar_info.width;
        // Get the height ratio then mutiply it to have height in screen
        let height = bar_info.height_in_screen * element.clone() as f32;

        let sprite = Sprite {
            color: Color::rgb_u8(255, 255, 255), //White
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite,
                transform: Transform {
                    // width and height have origin center should add off set to down left
                    translation: Vec3::new(width * index + width / 2., height / 2., 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Bar(array.value[index as usize]));

        index += 1.;
    }
}

// Update the bar state after being sorted
fn update_bar(array: Res<SortingArray>, bar_info: Res<BarInfo>,mut query: Query<(&mut Transform, &Bar), With<Bar>>) {
    let width = bar_info.width;
    // Get x, y and bar height
    for (mut transform, bar) in query.iter_mut() {
        // Check if current index array value is equal to bar height
        // If correct then draw it in correct index
        let mut index= 0;
        for element_index in 0..array.value.len() {
            if bar.0 == array.value[element_index] {
                index= element_index;
            }
        }

        // The correct index value and bar height
        transform.translation.x = width * index as f32 + width / 2.;
    }
}