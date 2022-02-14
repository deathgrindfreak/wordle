extern crate rand;
extern crate ctrlc;
extern crate wordle;

use rand::{thread_rng, Rng};
use std::{error::Error, process};

static WORD_FILE: &'static str = include_str!("../include/words");

fn main() {
    match pick_random_word() {
        Ok(word) => {
            let wordle_word = word.clone();
            ctrlc::set_handler(move || {
                println!("");
                println!("Such a quitter!");
                println!("Wordle word was: {}", wordle_word);
                process::exit(0);
            }).expect("Could not set Ctrl-C handler");

            wordle::play_game(word)
        },
        Err(error) => {
            println!("An error occurred: {}", error);
            process::exit(1);
        }
    }
}

fn pick_random_word() -> Result<String, Box<dyn Error>> {
    let words: Vec<&str> = WORD_FILE.lines().collect();

    let mut rnd = thread_rng();
    let i: usize = rnd.gen_range(0..words.len());

    Ok(words[i].to_string())
}
