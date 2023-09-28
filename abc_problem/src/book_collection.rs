pub struct Book {
    pub char: String,
    pub second_char: String,
    pub enable: bool
}

impl Book {
    const fn new(char: String, second_char: String) -> Book {
        Book {
            char,
            second_char,
            enable: true,
        }
    }
}

pub fn gen_book_collection() -> [Book; 20] {
    [
        Book::new("B".to_string(), "O".to_string()),
        Book::new("X".to_string(), "K".to_string()),
        Book::new("D".to_string(), "Q".to_string()),
        Book::new("C".to_string(), "P".to_string()),
        Book::new("N".to_string(), "A".to_string()),
        Book::new("G".to_string(), "T".to_string()),
        Book::new("R".to_string(), "E".to_string()),
        Book::new("T".to_string(), "G".to_string()),
        Book::new("Q".to_string(), "D".to_string()),
        Book::new("F".to_string(), "S".to_string()),
        Book::new("J".to_string(), "W".to_string()),
        Book::new("H".to_string(), "U".to_string()),
        Book::new("V".to_string(), "I".to_string()),
        Book::new("A".to_string(), "N".to_string()),
        Book::new("O".to_string(), "B".to_string()),
        Book::new("E".to_string(), "R".to_string()),
        Book::new("F".to_string(), "S".to_string()),
        Book::new("L".to_string(), "Y".to_string()),
        Book::new("P".to_string(), "C".to_string()),
        Book::new("Z".to_string(), "M".to_string()),
    ]
}
