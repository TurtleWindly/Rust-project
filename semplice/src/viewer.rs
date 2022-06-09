use std::{env, path::Path, process, fs};

#[derive(Default)]
pub struct Document {
    pub rows: Vec<String>,
}

pub fn set_startup_document() -> Document {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Need only one argument that is the file name");
        process::exit(1);
    }

    if file_not_exists(&args[1]) {
        println!("Can't find the file name");
        process::exit(1);
    }

    let content = fs::read_to_string(&args[1]).expect("Can't read file");

    seperate_lines(&content)
}

fn file_not_exists(file_name: &String) -> bool {
    !Path::new(file_name).exists()
}

fn seperate_lines(content: &str) -> Document {
    let mut document = Document::default();
    let mut line = String::new();
    let vector_chars: Vec<char> = content.chars().collect();

    for character in vector_chars {
        if character == '\n' {
            document.rows.push(line.clone());
            line = String::new();
            continue;
        }
        line.push_str(&character.to_string());
    }

    document
}
