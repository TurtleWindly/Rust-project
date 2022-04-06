use std::cmp::Ordering;

fn main() {
    let mut index = 1;
    loop {
        match index.cmp(&100) {
            Ordering::Less | Ordering::Equal => {
                match index % 3 == 0 && index % 5 == 0{
                    true  => println!("FizzBuzz"),
                    false => match index % 3 == 0 {
                        true  => println!("Fizz"),
                        false => match index % 5 == 0 {
                            true  => println!("Buzz"),
                            false => println!("{}", index),
                        }
                    }
                }
                index = index + 1;
            }
            Ordering::Greater => break,
        }
    }
}
