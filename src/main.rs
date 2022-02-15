extern crate rand;
extern crate ctrlc;
extern crate clap;

extern crate wordle;

use clap::Parser;
use rand::{thread_rng, Rng};
use std::{error::Error, process};

static WORD_FILE: &'static str = include_str!("../include/words");

#[derive(Parser, Debug)]
struct Args {
    /// Enables hard mode: Squares marked green and yellow must be used in subsequent answers.
    #[clap(long)]
    hard: bool,
}

fn main() {
    let args = Args::parse();
    let word_list: Vec<String> = WORD_FILE.lines().map(|s| s.to_string()).collect();

    match pick_random_word(&word_list) {
        Ok(word) => {
            let wordle_word = word.clone();
            ctrlc::set_handler(move || {
                println!("");
                println!("Such a quitter!");
                println!("Wordle word was: {}", wordle_word);
                process::exit(0);
            }).expect("Could not set Ctrl-C handler");

            wordle::play_game(wordle::Config::new(
                word,
                word_list.into_iter().collect(),
                args.hard
            ))
        },
        Err(error) => {
            println!("An error occurred: {}", error);
            process::exit(1);
        }
    }
}

fn pick_random_word(words: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut rnd = thread_rng();
    let i: usize = rnd.gen_range(0..words.len());

    Ok(words[i].to_string())
}
