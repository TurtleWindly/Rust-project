use rand::seq::SliceRandom;

fn gen_tmp_choices() -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut tmp_choices: Vec<usize> = (0..100).collect();
    tmp_choices.shuffle(&mut rng);
    let final_length = tmp_choices.len().saturating_sub(50);
    tmp_choices.truncate(final_length);
    return  tmp_choices;
}

fn gen_cupboard() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut cupboard: Vec<i32> = (1..=100).collect();
    cupboard.shuffle(&mut rng);
    return  cupboard;
}

fn prison_game() -> bool {
    let cupboard = gen_cupboard();

    let mut free_prisoners = 0;

    for prisoner in 1..=100 {
        let tmp_choices = gen_tmp_choices();
        for drawer in 0..50 {
            if prisoner == cupboard[tmp_choices[drawer]] {
                free_prisoners += 1;
                break;
            }
        }
    }

    return free_prisoners == 100;
}

fn prison_game_strategy() -> bool {
    let cupboard = gen_cupboard();

    let mut free_prisoners = 0;

    for prisoners in 1..=100 {
        let mut tmp_choice = prisoners;
        for _drawer in 0..50 {
            if prisoners == cupboard[tmp_choice as usize - 1] {
                free_prisoners += 1;
                break;
            } else {
                tmp_choice = cupboard[tmp_choice as usize - 1];
            }
        }
    }

    return free_prisoners == 100;
}

fn main() {
    let game_play = 100_000.0;
    let mut game_won = 0;

    // Without strategy
    println!("Prison game without strategy");
    for _game in 0..game_play as usize {
        if prison_game() { game_won += 1 }
    }
    println!("Game play: {}", game_play as i32);
    println!("Game won: {}", game_won);
    println!("Winrate: {}", game_won as f32 / game_play);

    // With strategy
    println!("Prison game with strategy");
    game_won = 0;

    for _game in 0..game_play as usize {
        if prison_game_strategy() { game_won += 1 }
    }
    println!("Game play: {}", game_play as i32);
    println!("Game won: {}", game_won);
    println!("Winrate: {}", game_won as f32 / game_play);
}
