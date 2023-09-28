use rand::seq::SliceRandom;

fn gen_tmp_choices() -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut tmp_choices: Vec<usize> = (0..100).collect();
    tmp_choices.shuffle(&mut rng);
    let final_length = tmp_choices.len().saturating_sub(50);
    tmp_choices.truncate(final_length);
    return  tmp_choices;
}

fn prison_game() -> bool {
    let mut rng = rand::thread_rng();
    let mut cupboard: Vec<i32> = (1..=100).collect();
    cupboard.shuffle(&mut rng);

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

fn main() {
    let game_play = 100_000.0;
    let mut game_won = 0.0;
    for _game in 0..game_play as usize {
        if prison_game() { game_won += 1.0 }
    }
    println!("Game play: {}", game_play as i32);
    println!("Game won: {}", game_won as i32);
    println!("Winrate: {}", game_won / game_play);
}
