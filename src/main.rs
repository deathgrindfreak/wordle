extern crate rand;

use rand::{thread_rng, Rng};
use std::{error::Error, io::{Write, stdin, stdout}, process};

static WORD_FILE: &'static str = include_str!("../include/words");

const GUESSES: i32 = 6;
const WORD_LENGTH: usize = 5;

fn main() {
    match pick_random_word() {
        Ok(word) => play_game(word),
        Err(error) => {
            println!("An error occurred: {}", error);
            process::exit(1);
        }
    }
}

fn play_game(word: String) {
    let mut guesses: i32 = 0;
    loop {
        match prompt_word() {
            Ok(guess) => {
                if guess.len() != WORD_LENGTH {
                    println!("Not a {} letter word.", WORD_LENGTH);
                } else {
                    guesses += 1;
                    let complete_match = determine_match(&word, &guess);
                    if complete_match {
                        println!("Correct!");
                        break;
                    } else if guesses == GUESSES {
                        println!("Wordle word was: {}", word);
                        break;
                    }
                }
            },
            Err(error) => {
                println!("Error occurred: {}", error);
                println!("Try again ...");
            }
        }
    }
}

fn determine_match(word: &String, guess: &String) -> bool {
    fn misplaced_char(word: &String, matches: &Vec<Option<char>>, g: char) -> bool {
        let appearances = word.chars().into_iter().filter(|&c| c == g).count();
        let matched = matches.iter().filter(|&o| o == &Some(g)).count();
        appearances > matched
    }

    let mut matches: Vec<Option<char>> = vec![None; WORD_LENGTH];
    for i in 0..WORD_LENGTH {
        let g = guess.chars().nth(i).unwrap();
        if word.chars().nth(i).unwrap() == g {
            matches[i] = Some(g);
            print!("🟩");
        } else if misplaced_char(&word, &matches, g) {
            print!("🟨");
        } else {
            print!("⬛");
        }
    }
    println!("");

    matches.iter().all(|&v| v.is_some())
}

fn prompt_word() -> Result<String, Box<dyn Error>> {
    let mut guess = String::new();

    print!("Guess: ");
    stdout().flush()?;
    stdin().read_line(&mut guess)?;

    Ok(guess.trim().to_string())
}

fn pick_random_word() -> Result<String, Box<dyn Error>> {
    let words: Vec<&str> = WORD_FILE.lines().collect();

    let mut rnd = thread_rng();
    let i: usize = rnd.gen_range(0..words.len());

    Ok(words[i].to_string())
}
