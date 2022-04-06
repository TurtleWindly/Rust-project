use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing game!");

    let secrect_number: u32 = rand::thread_rng().gen_range(1..101);

    // println!("The secrect_number is: {}", secrect_number);

    loop {
        println!("Please enter a guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Result failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secrect_number) {
            Ordering::Less    => println!("Too smol"),
            Ordering::Greater => {println!("Too big");}
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }

    }

}
