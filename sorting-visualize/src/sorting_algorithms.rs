use bevy::prelude::*;
use crate::sorting::SortingArray;

pub struct SortingAlgorithms;

impl Plugin for SortingAlgorithms {
    fn build(&self, app: &mut App) {
        app.add_system(sorting_arr.after("update"));
    }
}

// Buble sort
fn sorting_arr(mut array: ResMut<SortingArray>) {

    let len = array.value.len() as i32;

    // If array done sorting then stop
    if !array.is_sorting {
        return;
    }

    // Optimize loop times. For every loop times the last number will be correct then no need to check for it
    if array.sorted_times < len - array.loop_times - 1 {
        let index = array.sorted_times as usize;
        //Just compare the value
        if array.value[index] > array.value[index + 1] {
            //swap it
            let temp = array.value[index];
            array.value[index] = array.value[index + 1];
            array.value[index + 1] = temp;

            array.is_swapping = true;
        }
        array.sorted_times += 1;
    } else {
        // If not swapped last sort then it mean done
        if !array.is_swapping { array.is_sorting = false}
        // Done 1 loop
        array.is_swapping = false;
        array.sorted_times = 0;
        array.loop_times += 1;
    }

}