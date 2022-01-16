use rayon::prelude::*;

use crate::game::Word;

pub trait WordleAgent {
    fn select_move(&self, wordlist: &[Word]) -> Word;
}

// The goal of this agent is to find the word which breaks the possible responses into the most
// distinct groups
pub struct SplinterAgent;

fn score_word(test_word: &Word, wordlist: &[Word]) -> u32 {
        // Make an array with enough space to hold all possible (10 bit) responses. 
        // This is a bajillion times faster than a hashmap. (size = 2 ^ (WORD_SIZE * 2))
        let mut responses: [u16; 1024] = [0; 1024];
        for word in wordlist.iter() {
            let response = test_word.compare(word);
            responses[response] += 1;
        }

        let mut unique = 0;
        for response_tally in responses.iter() {
            if *response_tally != 0 {
                unique += 1
            }
        }
        return unique;
}

impl WordleAgent for SplinterAgent {
    fn select_move(&self, wordlist: &[Word]) -> Word {
        let scores: Vec<u32> = wordlist.par_iter()
                             .map(|test_word| {score_word(test_word, &wordlist)}).collect();


        let mut max_score = 0;
        let mut best_word = &wordlist[0];
        for (i, score) in scores.iter().enumerate() {
            if *score > max_score {
                best_word = &wordlist[i];
                max_score = *score;
            }
        }
        println!("{} yields {} unique responses", best_word.string, max_score);
        return best_word.clone();
    }
}

