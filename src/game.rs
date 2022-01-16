use std::convert::TryInto;
use std::fmt;

use crate::agent::WordleAgent;

pub const WORD_SIZE: usize = 5;

pub type WordleResponse = usize; //(u8, u8);

trait ColorResponse {
    fn green(&self) -> usize;
    fn yellow(&self) -> usize;
}

impl ColorResponse for WordleResponse {
    fn green(&self) -> usize {
        return self >> WORD_SIZE;
    }

    fn yellow(&self) -> usize {
        return self & 0b1111;
    }
}

impl fmt::Display for dyn ColorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let yellow = self.yellow();
        let green = self.green();

        let mut repr = String::new();

        for i in (0..WORD_SIZE).rev() {
            if (green >> i) & 1 != 0 {
                repr.push('🟩');
            }
            else if (yellow >> i) & 1 != 0 {
                repr.push('🟨');
            }
            else {
                repr.push('⬛');
            }
        }

        write!(f, "{}", repr)
    }
}

#[derive(Clone)]
pub struct Word {
    pub string: String,
    bytes: [u8; WORD_SIZE],
    mask: u32,
}

impl Word {
    pub fn compare(&self, other: &Word) -> WordleResponse {
        let letters_shared = other.mask & self.mask;
        let mut response: usize = 0;

        for (i, c) in self.bytes.iter().enumerate() {
            if *c == other.bytes[i] {
                response |= 1 << (WORD_SIZE - 1 - i + WORD_SIZE);
            }
            else if letters_shared & (1 << (25 - (c - b'a'))) != 0 {
                response |= 1 << (WORD_SIZE - 1 - i);
            }
        }

        return response
    }

    pub fn from_str(str_val: &str) -> Word {
        let bytes: [u8; WORD_SIZE] = str_val.as_bytes().try_into().expect("Error converting string into bytes array");
        let mask = get_wordmask(str_val);

        return Word{ string: str_val.to_string(), bytes, mask }
    }
}


fn get_wordmask(word: &str) -> u32 {
    let mut wordmask: u32 = 0;
    for c in word.chars() {
        let letter_offset = c as u32 - 'a' as u32;
        wordmask |= 1 << (25 - letter_offset);
    }
    return wordmask;
}

fn eliminate(guess: &Word, response: WordleResponse, wordlist: &[Word]) -> Vec<Word> {
    let mut new_wordlist = vec![];
    for word in wordlist.iter() {
        if guess.compare(word) == response {
            new_wordlist.push(word.clone());
        }
    }
    return new_wordlist;
}

pub fn play(solution: Word, agent: &dyn WordleAgent, wordlist: Vec<Word>, guesses: u8, initial_guess: Option<Word>) -> bool {

    if guesses == 0 {
        println!("GAME OVER! The word was {}", solution.string);
        return false;
    }

    let guess = match initial_guess {
        Some(x) => x,
        None => agent.select_move(&wordlist),
    };


    println!("guessing {}", guess.string);
    let response = guess.compare(&solution);
    println!("response was {}", &response as &dyn ColorResponse);

    if response >> WORD_SIZE == 0b11111 {
        println!("I win! The word is {}", guess.string);
        return true;
    }
    let new_wordlist = eliminate(&guess, response, &wordlist);
    println!("new wordlist contains {} words", new_wordlist.len());

    return play(solution, agent, new_wordlist, guesses - 1, None);
}
