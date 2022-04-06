fn number_to_day(number: usize) -> String {
    match number {
        1 => return "st".to_string(),
        2 => return "nd".to_string(),
        3 => return "rd".to_string(),
        _ => return "th".to_string(),
    }
}

fn main() {
    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let mut today: usize = 1;
    // let mut quotes: [&str; 12];

    for _quote in gifts {
        println!(
            "\nOn the {}{} Days of Chrismas",
            today,
            number_to_day(today)
        );
        println!("My true love gave me:");
        for sing in (0..today).rev() {
            println!("{}", gifts[sing]);
        }
        today = today + 1;
    }
}
