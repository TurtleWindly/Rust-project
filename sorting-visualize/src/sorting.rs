use bevy::prelude::*;

#[derive(Default)]
struct SortingArray {
    value: Vec<i32>,
    loop_times: i32,
    sorted_times: i32,
    // Check if arr need swapping, if not is sorting will false meaning the arr is done
    is_swapping: bool,
    is_sorting: bool,
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
            value: vec![9, 2, 4, 7, 0, 1, 6, 3, 5, 8],
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
        .add_system(sorting_arr);
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
            .insert(Bar);

        index = index + 1.;
    }
}

//Buble Sort
// Sorting alrorithm but NOT DRAWING!
fn sorting_arr(mut array: ResMut<SortingArray>) {

    let len = array.value.len() as i32;

    if !array.is_sorting {
        return;
    }

    if array.sorted_times < len - array.loop_times - 1 {
        let index = array.sorted_times as usize;
        if array.value[index] > array.value[index + 1] {
            let temp = array.value[index];
            array.value[index] = array.value[index + 1];
            array.value[index + 1] = temp;

            array.is_swapping = true;
        }
        array.sorted_times += 1;
    } else {
        if !array.is_swapping { array.is_sorting = false}
        array.is_swapping = false;
        array.sorted_times = 0;
        array.loop_times += 1;
    }

}