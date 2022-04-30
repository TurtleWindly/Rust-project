use colored::*;

fn main() {
    let mut red: u8   = 255;
    let mut green: u8 = 128;
    let mut blue: u8  = 0;

    let mut timer = 0;

    let mut counter = true;
    let mut midle_counter = true;

    loop {
        if timer < 500000 {
            timer += 1;
            continue;
        } else {
            timer = 0;
        }
        clearscreen::clear().expect("Failed to clear screen");
        println!("{}", "Hello, world!".truecolor(red, green, blue));

        if counter {
            if red == 0 { counter = !counter; continue; }
            red -= 1;
            blue += 1;
        } else {
            if red == 255 { counter = !counter; continue; }
            red += 1;
            blue -= 1;
        }
        if midle_counter {
            if green == 0 { midle_counter = !midle_counter; continue;}
            green -= 1;
        } else {
            if green == 255 { midle_counter = !midle_counter; continue;}
            green += 1;
        }
    }
}
