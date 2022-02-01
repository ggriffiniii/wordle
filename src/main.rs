use wordle::{Game, Hint};

fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    loop {
        let next_guess = game.best_guess();
        println!("guess: {}", next_guess);
        for (idx, letter) in next_guess.bytes().enumerate() {
            let hint = prompt_for_hint(idx, letter);
            game.add_hint(letter, hint);
        }
    }
}

fn prompt_for_hint(idx: usize, letter: u8) -> Hint {
    loop {
        println!(
            "what result did you get for '{}'? (! not in word/g green/y yellow)",
            letter as char
        );
        use std::io::{self, BufRead};
        match io::stdin().lock().lines().next().unwrap().unwrap().as_str() {
            "!" => return Hint::NotInWord,
            "g" => return Hint::Correct(idx),
            "y" => return Hint::WrongSpot(idx),
            _ => {
                println!("invalid entry; try again");
            }
        }
    }
}
