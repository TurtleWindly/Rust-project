use std::{ env, process };

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() <= 1usize {
        process::exit(1);
    }

    args.remove(0);

    for argument in args {
        print!("{} ", argument);
    }
    println!();
}
