mod game;
mod agent;

use game::{play, Word, WORD_SIZE};
use agent::{SplinterAgent, AverseAgent};

extern crate rand;
extern crate clap;
extern crate random_string;
use rand::seq::SliceRandom;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

const MAX_GUESSES: u8 = 6;

#[derive(Parser)]
struct CliArgs {
    /// number of games to simulate
    #[clap(short, long, default_value_t = 1)]
    plays: u32,
    /// initial word to use as a guess (e.g. if it's been precomputed)
    #[clap(short, long)]
    initial_guess: Option<String>,
    /// number of guesses allowed per game
    #[clap(short, long, default_value_t = MAX_GUESSES)]
    max_guesses: u8,
    /// path to a txt file containing the list of words to use
    #[clap(short, long, default_value = "5_letter_words.txt")]
    wordlist: String,
    /// word to use as the solution (by default a random word is chosen from the word list)
    #[clap(short, long)]
    solution: Option<String>,
}

fn load_wordlist(filename: std::path::PathBuf) -> Vec<Word> {
    println!("Loading wordlist from {}", filename.to_string_lossy());
    let contents = fs::read_to_string(filename)
        .expect("error while reading wordlist file");

    let mut wordlist: Vec<Word> = vec![];
    for line in contents.lines() {
        wordlist.push(Word::from_str(line));
    }

    return wordlist;
}

fn random_wordlist(wordlen: usize, n: usize) -> Vec<Word> {
    let charset: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut wordlist: Vec<Word> = Vec::with_capacity(n);
    for _ in 0..n {
        let word = random_string::generate(wordlen, charset);
        wordlist.push(Word::from_str(&word));
    }

    return wordlist;
}

fn main() {
    let args = CliArgs::parse();

    let words = random_wordlist(WORD_SIZE, 10000);//load_wordlist(PathBuf::from(args.wordlist));

    let agent = AverseAgent{};
    let initial_guess = match args.initial_guess {
        Some(x) => Some(Word::from_str(&x)),
        None => None,
    };

    let mut wins: u32 = 0;
    for _ in 0..args.plays {
        let solution = match args.solution {
            Some(ref x) => Word::from_str(x),
            None => words.choose(&mut rand::thread_rng()).unwrap().clone()
        };
        //let solution = Word::from_str("dwarf");
        if play(solution, &agent, words.clone(), MAX_GUESSES, initial_guess.clone()) {
            wins += 1;
        }
    }

    let win_rate: f32 = wins as f32 / args.plays as f32;
    println!("Played {} total games. Won {}. ({:.2}%)", args.plays, wins, win_rate * 100.0);
}
