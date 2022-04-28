use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();

    if !(args.len() == 3usize) {
        println!("Need correct 2 arguments: action and file");
        process::exit(1);
    }

    let action = checking_what_action(&args[1]);

    if file_not_exists(&args[2]) {
        println!("Can't find file");
        process::exit(1);
    }

    let file_name = &args[2];
    let contents = fs::read_to_string(file_name)
     .expect("Something went wrong, can't read file");

    let mut file = File::create(format!("file.{}", action))?;

    match action.as_str() {
        "encode" => encode(&contents, &mut file),
        "decode" => decode(&contents, &mut file),
        _ => (),
    }

    Ok(())
}

fn encode(contents: &String, file: &mut File) {
    let vector_chars: Vec<char> = contents.chars().collect();
    let mut encoded_array: Vec<u8> = Vec::new();
    let mut encoded_string: String = String::new();

    for character in vector_chars {
        encoded_array.push(character as u8);
    }

    for encoded_number in encoded_array {
        if encoded_number == 32 {
            encoded_string.push_str("a");
            continue;
        }
        encoded_string.push_str(&encoded_number.to_string());
        encoded_string.push_str("z");
    }

    write!(file, "{}", encoded_string).expect("can't write to file");

    println!("encoded");
}

fn decode(contents: &String, file: &mut File) {
    let vector_chars: Vec<char> = contents.chars().collect();
    let mut decoded_string = String::new();
    let mut index_encoded_string = String::new();

    for character in vector_chars {
        if character == 'z' {
            let ascii_code: u8 = index_encoded_string.parse().unwrap();
            let char_from_ascii_code = ascii_code as char;
            decoded_string.push_str(&char_from_ascii_code.to_string());
            index_encoded_string = String::new();
            continue;
        }
        if character == 'a' {
            decoded_string.push_str(" ");
            continue;
        }
        index_encoded_string.push_str(&character.to_string());
    }

    write!(file, "{}", decoded_string).expect("can't write to file");

    println!("decoded");
}

fn checking_what_action(action: &String) -> String {
    match action.as_str() {
     "encode" => String::from("encode"),
     "decode" => String::from("decode"),
     _ => {
         println!("Unknow action");
         println!("Possible action is 'encode' and 'decode'");
         process::exit(1);
     }
    }
}

fn file_not_exists(file_name: &String) -> bool {
!Path::new(file_name).exists()
}
