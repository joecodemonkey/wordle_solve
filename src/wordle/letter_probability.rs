use std::default::Default;
use std::collections::HashSet;
use crate::wordle::{Board, MAX_ATTEMPTS, MAX_LETTERS, Word};

#[derive(Debug, Clone)]
pub struct LetterProbability {
    counts: Vec<std::collections::HashMap<char, u64>>,
    word_count: u32
}

impl Default for LetterProbability {
    fn default() -> Self {
        Self {
            counts:  vec![Default::default(); crate::wordle::word::MAX_LETTERS],
            word_count: 0u32
        }
    }
}

impl LetterProbability {
    pub fn add_word(self: &mut Self, word: &String){
        if word.len() > self.counts.len() {
            panic!("Word Length must not exceed set length of {} characters", MAX_LETTERS);
        }
        for (index, letter) in word.chars().enumerate() {
            self.add_letter(&letter, index);
        }
    }

    fn add_letter(self: &mut Self, letter: &char, index: usize) {
        let &mut mut map = self.counts.iter_mut().nth(index).unwrap();
        let &letter = &letter.to_lowercase().nth(0).unwrap();
        if let Some(count) = map.get_mut(&letter) {
            *count = *count + 1;
        }
        else {
            map.insert(letter.clone(), 1);
        }
    }

    pub fn score_word(self: &Self, word: &String) -> f64 {
        let mut ret = 0.0f64;
        for (index, letter) in word.chars().enumerate() {
            let map = self.counts.iter().nth(index).unwrap();
            let value = map.get(&letter);
            match value {
                Some(value) => {
                    let mut num : f64 = *value as f64;
                    ret += num / self.word_count as f64;
                }
                None => { return 0.0f64; }
            }
        }
        ret
    }
}
