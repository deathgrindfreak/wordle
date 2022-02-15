use std::{error::Error, io::{Write, stdin, stdout}};
use std::collections::HashSet;

const GUESSES: i32 = 6;
const WORD_LENGTH: usize = 5;

#[derive(Debug)]
pub struct HardMatches {
    pub greens: Vec<(char, usize)>,
    pub yellows: HashSet<char>,
}

#[derive(Debug)]
pub struct Config {
    pub hard: bool,
    pub word: String,
    words: HashSet<String>,
    hard_matches: Option<HardMatches>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchType {
    Match,
    Misplaced,
    NoMatch,
}

impl Config {
    pub fn new(word: String, words: HashSet<String>, hard: bool) -> Self {
        Config {word, words, hard, hard_matches: None}
    }

    pub fn in_list(&self, guess: &str) -> bool {
        self.words.contains(guess)
    }

    pub fn is_good_hard_mode_guess(&self, guess: &String) -> bool {
        match &self.hard_matches {
            Some(HardMatches {greens, yellows}) => {
                let greens_idx: Vec<usize> = greens.iter().map(|(_, i)| *i).collect();
                let leftover_chars: Vec<char> = guess.chars().zip(0..)
                                                             .filter(|(_, i)| !greens_idx.contains(&i))
                                                             .map(|(g, _)| g)
                                                             .collect();

                greens.iter().all(|(c, i)| *c == guess.chars().nth(*i).unwrap())
                    && yellows.iter().all(|y| leftover_chars.contains(y))
            },
            None => true // Haven't guess yet
        }
    }

    pub fn determine_match(&mut self, guess: &String) -> Vec<MatchType> {
        let mut matches: Vec<MatchType> = Vec::new();
        let mut leftover_chars = String::new();
        let mut greens: Vec<(char, usize)> = Vec::new();
        for ((w, g), i) in self.word.chars().zip(guess.chars()).zip(0..) {
            if w == g {
                greens.push((w, i));
                matches.push(MatchType::Match);
            } else {
                leftover_chars.push(w);
                matches.push(MatchType::NoMatch);
            }
        }

        let mut yellows = HashSet::new();
        for (g, i) in guess.chars().zip(0..){
            if matches[i] == MatchType::NoMatch && leftover_chars.contains(g) {
                matches[i] = MatchType::Misplaced;
                yellows.insert(g);
            }
        }

        self.hard_matches = Some(HardMatches { greens, yellows });

        matches
    }
}

pub fn play_game(mut config: Config) {
    let mut guesses: i32 = 0;

    if config.hard {
        println!("Hard mode enabled!");
    }

    loop {
        match prompt_word() {
            Ok(guess) => {
                if guess.len() != WORD_LENGTH {
                    println!("Not a {} letter word.", WORD_LENGTH);
                } else if !config.in_list(&guess.as_str()) {
                    println!("Word is not a word in the list");
                } else {
                    if config.hard && !config.is_good_hard_mode_guess(&guess) {
                        println!("Guess did not contain green and yellow letters from previous guess.");
                        continue;
                    }

                    guesses += 1;

                    let matches = config.determine_match(&guess);
                    print_match(&matches);

                    if matches.iter().all(|&m| m == MatchType::Match) {
                        println!("Correct!");
                        break;
                    } else if guesses == GUESSES {
                        println!("Wordle word was: {}", config.word);
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
            test_match("robot", "robot"),
            vec![Match; WORD_LENGTH]
        );
    }

    #[test]
    fn test_robot_robto() {
        assert_eq!(
            test_match("robot", "robto"),
            vec![Match, Match, Match, Misplaced, Misplaced]
        );
    }

    #[test]
    fn test_spine_every() {
        assert_eq!(
            test_match("spine", "every"),
            vec![Misplaced, NoMatch, Misplaced, NoMatch, NoMatch]
        );
    }

    #[test]
    fn test_robot_roboo() {
        assert_eq!(
            test_match("robot", "rooot"),
            vec![Match, Match, NoMatch, Match, Match]
        );
    }

    #[test]
    fn test_rebus_arise() {
        assert_eq!(
            test_match("rebus", "arise"),
            vec![NoMatch, Misplaced, NoMatch, Misplaced, Misplaced]
        );
    }

    #[test]
    fn test_rebus_route() {
        assert_eq!(
            test_match("rebus", "route"),
            vec![Match, NoMatch, Misplaced, NoMatch, Misplaced]
        );
    }

    #[test]
    fn test_rebus_rules() {
        assert_eq!(
            test_match("rebus", "rules"),
            vec![Match, Misplaced, NoMatch, Misplaced, Match]
        );
    }

    fn test_match(word: &str, guess: &str) -> Vec<MatchType> {
        let w = word.to_string();
        let g = guess.to_string();
        Config::new(w.clone(), HashSet::from([w.clone(), g.clone()]), false)
            .determine_match(&g.clone())
    }
}
