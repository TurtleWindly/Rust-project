fn toogle_doors(step: usize, mut doors: [bool; 100]) -> [bool; 100] {
    for index in (0..=doors.len()).step_by(step) {
        doors[index] = !doors[index];
        println!("{}", doors[index]);
    }
    return doors;
}

fn main() {
    let mut doors: [bool; 100] = [false; 100];
    for index in 1..=doors.len() {
        doors = toogle_doors(index, doors);
    }
    for (index, door) in doors.iter().enumerate() {
        if *door {
            println!("The {} door is open", index);
        }
    }
}
