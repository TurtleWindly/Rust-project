use std::io;

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32f32) * 5f32 / 9f32
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9f32 / 5f32 + 32f32
}

fn main() {

    println!("Input temperature: ");
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_)  => 0f32,
    };

    println!("This is C or F ? ");

    let mut format = String::new();

    io::stdin()
        .read_line(&mut format)
        .expect("Failed to read line");
    let format = format.trim();

    if format == "C" {
        println!("to fahrenheit is {}", to_fahrenheit(temperature));
    } else {
        println!("to celius is {}", to_celsius(temperature));
    }

}
