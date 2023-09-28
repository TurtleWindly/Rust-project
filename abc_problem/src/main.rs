use book_collection::gen_book_collection;

mod book_collection;
mod test;

pub fn can_make_word(word: String) -> bool {
    let mut book_collection = gen_book_collection();

    for character in word.chars() {
        let mut is_available = false;
        for book in &mut book_collection {
            if !book.enable {continue;}
            if character.to_string() == book.char || character.to_string() == book.second_char {
                is_available = true;
                book.enable = false;
                break;
            }
        }
        if !is_available {return false;}
    }
    return true;
}

fn main() {
    println!("{}", can_make_word("SQUAD".to_string()));
}
