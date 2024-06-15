use std::collections::hash_set::HashSet;
use crate::wordle;
use crate::wordle::letter_probability::LetterProbability;
use super::{Letter, MAX_LETTERS, Word};

#[derive(Debug, Clone)]
pub struct Statistics {
    all_words: HashSet<String>,
    pub filters: Vec<Word>,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            all_words: Default::default(),
            filters: Default::default()
        }
    }
}

impl Statistics {
    pub fn add_word(self: &mut Self, word: &String) {
        if word.len() > MAX_LETTERS {
            panic!("Word {} of length {} is greater than allowed length {}", word, word.len(), MAX_LETTERS)
        }

        self.all_words.insert(word.clone());
    }

    pub fn add_words(self: &mut Self, words: &Vec<String>) {
        for word in words {
            self.all_words.insert(word.to_string());
        };
    }

    pub fn guess(self: &mut Self) -> String {
        let mut probability = LetterProbability::default();
        for word in self.all_words.iter() {
            if (!self.filtered(word)) {
                probability.add_word(&word.clone());
            }
        }

        let mut guessed_word : String = String::from("");
        let mut guessed_word_score : f64 = 0.0f64;


        for word in self.all_words.iter() {
            let mut score = probability.score_word(&word.clone());
            if score > guessed_word_score {
                guessed_word_score = score;
                guessed_word = word.clone();
            }
        }

        guessed_word
    }

    fn filtered(self: &mut Self, word: &String) -> bool {
        for filter in self.filters.iter() {
            if filter.filter(word) {
                return true;
            }
        }
        false
    }

    pub fn clear(self: &mut Self) {
        self.all_words.clear();
    }
}
