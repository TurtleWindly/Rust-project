use std::fs::File;

// Get input
use std::io;
use std::io::Write;

// Hashing part
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ron
use ron::de::from_reader;
use serde::Deserialize;

#[derive(Hash, Deserialize)]
struct User {
    name: String,
    pass: u64,
}

#[derive(Deserialize)]
struct List {
    user_list: Vec<User>,
}

fn main() {
    // Get data from user.ron
    let input_path = String::from("src/user.ron");
    let f = File::open(&input_path).expect("Failed opening file");
    let list: List = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    let login_name = name_login();
    // Check login name
    check_name(&list, login_name);

    let login_pass = pass_login();
    // hah the pass
    let hashed_pass = calculate_hash(&login_pass);
    // Check the password
    check_pass(&list, hashed_pass);

    println!("Corract you have login successfully !");

}

fn name_login() -> String {
    let mut name = String::new();
    println!("---------------------------------------------------------");
    print!("login: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Can't read input");
    // remove \n
    name.pop();
    // then return name
    name
}

fn pass_login() -> String {
    let mut pass = String::new();
    print!("Password: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut pass)
        .expect("Can't read input");
    // remove \n
    pass.pop();
    // then return pass
    pass
}

fn check_name(list: &List, name: String) {
    let mut name_exsit = false;
    for user in &list.user_list {
        if name == user.name {
            name_exsit = true;
        }
    }

    // if name not exsit then quit
    if !name_exsit {
        println!("Name do not exsit");
        std::process::exit(0);
    }
}

fn check_pass(list: &List, hashed_pass: u64) {
    let mut correct_pass = false;
    for user in &list.user_list {
        if hashed_pass == user.pass {
            correct_pass = true;
        }
    }

    // if wrong password then exit
    if !correct_pass {
        println!("Wrong password");
        std::process::exit(0);
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
