use rayon::prelude::*;

use crate::game::Word;

pub trait WordleAgent {
    fn select_move(&self, wordlist: &[Word]) -> Word;
}


// The goal of this agent is to find the word which breaks the possible responses into the most
// distinct groups
pub struct SplinterAgent {
    pub first_guess: Option<Word>,
}

// Similar to the SplinterAgent, this one tries to break the possible responses into the most
// groups. However, this one also tries to minimize the average number of words in the remaining
// groups.
pub struct AverseAgent;


impl SplinterAgent {

    fn score_word(&self, test_word: &Word, wordlist: &[Word]) -> u32 {
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
}

impl WordleAgent for SplinterAgent {
    fn select_move(&self, wordlist: &[Word]) -> Word {
        let scores: Vec<u32> = wordlist.par_iter()
                             .map(|test_word| {self.score_word(test_word, &wordlist)}).collect();


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

impl AverseAgent {
    fn score_word(&self, test_word: &Word, wordlist: &[Word]) -> (u32, f32, u32) {
            // Make an array with enough space to hold all possible (10 bit) responses. 
            // This is a bajillion times faster than a hashmap. (size = 2 ^ (WORD_SIZE * 2))
            let mut responses: [u32; 1024] = [0; 1024];
            for word in wordlist.iter() {
                let response = test_word.compare(word);
                responses[response] += 1;
            }

            let mut unique = 0;
            let mut sum = 0;
            let mut worst = 0;
            for response_tally in responses.iter() {
                sum += response_tally;
                if *response_tally != 0 {
                    unique += 1;
                }
                if *response_tally > worst {
                    worst = *response_tally;
                }
            }
            let average: f32 = sum as f32 / unique as f32;

            return (unique, average, worst);
    }
}

impl WordleAgent for AverseAgent {
    fn select_move(&self, wordlist: &[Word]) -> Word {
        let scores: Vec<(u32, f32, u32)> = wordlist.par_iter()
                             .map(|test_word| {self.score_word(test_word, &wordlist)}).collect();


        let mut max_u = 0.0;
        let mut max_a = 0.0;
        let mut max_w = 0.0;
        for (unique, average, worst) in scores.iter() {
            if *unique as f32 > max_u {
                max_u = *unique as f32;
            }
            if *average > max_a {
                max_a = *average;
            }
            if *worst as f32 > max_w {
                max_w = *worst as f32;
            }
        }
        let final_scores: Vec<f32> = scores.iter().map(|(unique, average, worst)| {
            let uniqueness = (*unique as f32) / max_u;
            let average_case = (*average as f32) / max_u;
            let worst_case = (*worst as f32) / max_w;

            const ENTROPY: f32 = 1.0;
            const RISK_AVERSE: f32 = 0.5;
            const WORST_CASE_AVERSE: f32 = 0.2;

            return uniqueness * ENTROPY - average_case * RISK_AVERSE - worst_case * WORST_CASE_AVERSE;
        }).collect();


        let mut max_score = 0.0;
        let mut best_word = &wordlist[0];
        for (i, score) in final_scores.iter().enumerate() {
            if *score > max_score {
                best_word = &wordlist[i];
                max_score = *score;
            }
        }
        return best_word.clone();
    }
}
