use std::io;

fn fibonacci(number: i32) -> i32 {
    if number <= 1 {return number}
    fibonacci(number - 1) + fibonacci(number - 2)
}

fn main() {
    println!("Input a number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    println!("Output: {}", fibonacci(input.trim().parse().expect("Input number pls")));
}
