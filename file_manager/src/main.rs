use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Can't find home directory"),
    };

    let mut path = home.clone();

    loop {
        print_dir(&path);

        let index = get_input();

        if index == 0 {
            go_back_dir(&mut path);
            continue;
        }

        match go_to_dir(index, get_dir(&path)) {
            Some(current_path) => path = current_path,
            None => println!("Can't file dir"),
        }

    }
}

fn get_input() -> usize {
    let mut input = String::new();
    println!("Input: ");
    io::stdin()
        .read_line(&mut input)
        .expect("can't read input");
    input.trim().parse::<usize>().unwrap()
}

fn print_dir(path: &Path) {
    let mut index = 1;
    println!("0 Go back");
    for entry in path.read_dir().expect("read dir call failed") {
        if let Ok(entry) = entry {
            println!("{} {}", index, entry.path().file_name().unwrap().to_str().unwrap());
        }
        index += 1;
    }
}

fn get_dir(path: &PathBuf) -> Vec<PathBuf> {
    let mut options: Vec<PathBuf> = Vec::new();
    for entry in path.read_dir().expect("read dir call failed") {
        if let Ok(entry) = entry {
            options.push(entry.path());
        }
    }

    options
}

fn go_to_dir(index: usize, options: Vec<PathBuf>) -> Option<PathBuf> {
    match options.get(index - 1) {
        Some(path) => Some(path.clone()),
        None => None,
    }
}

fn go_back_dir(path: &mut PathBuf) {
    path.pop();
}
