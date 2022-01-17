mod game;
mod agent;

use game::{play, Word, WORD_SIZE};
use agent::{WordleAgent, SplinterAgent, AverseAgent};

extern crate rand;
extern crate clap;
extern crate random_string;
use rand::seq::SliceRandom;
use clap::{Parser, ArgEnum};
use std::fs;
use std::path::PathBuf;

const MAX_GUESSES: u8 = 6;

#[derive(Clone, Debug, ArgEnum)]
enum AgentType {
    Splinter,
    Averse,
}

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
    /// whether or not to use randomly generated words instead of a real wordlist
    #[clap(short, long)]
    random_wordlist: bool,
    /// the type of agent to run with
    #[clap(short, long, arg_enum, default_value = "splinter")]
    agent_type: AgentType,
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

    let words = if args.random_wordlist {
        random_wordlist(WORD_SIZE, 10000)//load_wordlist(PathBuf::from(args.wordlist));
    } else {
        load_wordlist(PathBuf::from(args.wordlist))
    };

    let initial_guess = match args.initial_guess {
        Some(x) => Some(Word::from_str(&x)),
        None => None,
    };

    // TODO: The easiest way I've found so far to dynamically choose the agent implentation was to
    // allocate space for both of them. I think there's a better way somehow (enums?)
    let mut splinter_agent = SplinterAgent{first_guess: initial_guess};
    let mut averse_agent = AverseAgent{};

    let agent: &mut dyn WordleAgent = match args.agent_type {
        AgentType::Splinter => {
            println!("Running with the splinter agent (maximizing entropy)");
            &mut splinter_agent
        },
        AgentType::Averse => {
            println!("Running with the risk-averse agent (maximizing entropy and avoiding worst-cases)");
            &mut averse_agent
        },
    };

    let mut wins: u32 = 0;
    for _ in 0..args.plays {
        let solution = match args.solution {
            Some(ref x) => Word::from_str(x),
            None => words.choose(&mut rand::thread_rng()).unwrap().clone()
        };
        //let solution = Word::from_str("dwarf");
        if play(solution, agent, words.clone(), args.max_guesses, true) {
            wins += 1;
        }
    }

    let win_rate: f32 = wins as f32 / args.plays as f32;
    println!("Played {} total games. Won {}. ({:.2}%)", args.plays, wins, win_rate * 100.0);
}
