fn main() {
    let mut doors = [false; 100];
    let mut _index: usize = 0;
    for step in 0..doors.len() {
        _index = step;
        while _index < 100 {
            doors[_index] = !doors[_index];
            _index += step + 1;
        }
    }

    // Output
    for (index, door) in doors.iter().enumerate() {
        if *door {
            println!("The {} door is open", index + 1);
        }
    }
}
