use std::fs;
// use std::fs::File;
// use std::io::prelude::*;

mod first_time;

pub const CONFIG_DIR: &str= "~/.config/spanagerment";
pub const CONFIG_FILE : &str= "~/.config/spanagerment/config.ron"; 

fn main() {
    match fs::metadata(CONFIG_FILE) {
        Ok(_) => println!("File exsits"),
        Err(_) => println!("File does not exsits"),
    }
}
