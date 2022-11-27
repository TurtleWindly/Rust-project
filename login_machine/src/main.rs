use std::io;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;

#[derive(Hash)]
struct User {
    name: String,
    pass: String,
}

fn main() {
    // get login name input
    let login_name = name_login();
    // check if the name exsit
    // if not then quit
    println!("{}", ron_to_string());

    //TODO get login pass input
    // check if the pass exsit
    // if not then quit
    // if yes then print corract then end
}

fn ron_to_string() -> String {
    let ron = fs::read_to_string("user.ron")
        .expect("Can't read user.ron");
    ron
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn name_login() -> String {
    let mut name = String::new();
    println!("---------------------------------------------------------");
    print!("login: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Can't read input");
    name
}
