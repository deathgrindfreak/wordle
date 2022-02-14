use std::{error::Error, io::{Write, stdin, stdout}};

const GUESSES: i32 = 6;
const WORD_LENGTH: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MatchType {
    Match,
    Misplaced,
    NoMatch,
}

pub fn play_game(word: String, word_list: Vec<&str>) {
    let mut guesses: i32 = 0;
    loop {
        match prompt_word() {
            Ok(guess) => {
                if guess.len() != WORD_LENGTH {
                    println!("Not a {} letter word.", WORD_LENGTH);
                } else if !word_list.contains(&guess.as_str()) {
                    println!("Word is not a word in the list");
                } else {
                    guesses += 1;
                    let matches = determine_match(&word, &guess);

                    print_match(&matches);
                    if matches.iter().all(|&m| m == MatchType::Match) {
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

fn determine_match(word: &String, guess: &String) -> Vec<MatchType> {
    let mut matches: Vec<MatchType> = vec![];
    let mut leftover_chars = String::new();
    for (w, g) in word.chars().zip(guess.chars()) {
        matches.push(
            if w == g {
                MatchType::Match
            } else {
                leftover_chars.push(w);
                MatchType::NoMatch
            }
        );
    }

    for (g, i) in guess.chars().zip(0..){
        if matches[i] == MatchType::NoMatch && leftover_chars.contains(g) {
            matches[i] = MatchType::Misplaced;
        }
    }

    matches
}

fn print_match(matches: &Vec<MatchType>) {
    for m in matches {
        match m {
            MatchType::Match => print!("🟩"),
            MatchType::Misplaced => print!("🟨"),
            MatchType::NoMatch => print!("⬛"),
        }
    }
    println!("");
}

fn prompt_word() -> Result<String, Box<dyn Error>> {
    let mut guess = String::new();

    print!("Guess: ");
    stdout().flush()?;
    stdin().read_line(&mut guess)?;

    Ok(guess.trim().to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use MatchType::{Match, Misplaced, NoMatch};

    #[test]
    fn test_equal_strings() {
        assert_eq!(
            determine_match(&String::from("robot"), &String::from("robot")),
            vec![Match; WORD_LENGTH]
        );
    }

    #[test]
    fn test_robot_robto() {
        assert_eq!(
            determine_match(&String::from("robot"), &String::from("robto")),
            vec![Match, Match, Match, Misplaced, Misplaced]
        );
    }

    #[test]
    fn test_spine_every() {
        assert_eq!(
            determine_match(&String::from("spine"), &String::from("every")),
            vec![Misplaced, NoMatch, Misplaced, NoMatch, NoMatch]
        );
    }

    #[test]
    fn test_robot_roboo() {
        assert_eq!(
            determine_match(&String::from("robot"), &String::from("rooot")),
            vec![Match, Match, NoMatch, Match, Match]
        );
    }

    #[test]
    fn test_rebus_arise() {
        assert_eq!(
            determine_match(&String::from("rebus"), &String::from("arise")),
            vec![NoMatch, Misplaced, NoMatch, Misplaced, Misplaced]
        );
    }

    #[test]
    fn test_rebus_route() {
        assert_eq!(
            determine_match(&String::from("rebus"), &String::from("route")),
            vec![Match, NoMatch, Misplaced, NoMatch, Misplaced]
        );
    }

    #[test]
    fn test_rebus_rules() {
        assert_eq!(
            determine_match(&String::from("rebus"), &String::from("rules")),
            vec![Match, Misplaced, NoMatch, Misplaced, Match]
        );
    }
}
