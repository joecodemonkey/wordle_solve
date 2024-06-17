use std::collections::hash_set::HashSet;
use crate::wordle::letter_probability::LetterProbability;
use super::{MAX_LETTERS, Word};

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
    pub fn len(self: &Self) -> usize {
        self.all_words.len()
    }

    pub fn add_word(self: &mut Self, word: &String) {
        if word.len() > MAX_LETTERS {
            panic!("Word {} of length {} is greater than allowed length {}", word, word.len(), MAX_LETTERS)
        }

        self.all_words.insert(word.clone());
    }

    pub fn guess(self: &mut Self) -> String {
        let mut probability = LetterProbability::default();
        for word in self.all_words.iter() {
            if  !self.filtered(word) {
                probability.add_word(&word.clone());
            }
        }

        let mut guessed_word : String = String::from("");
        let mut guessed_word_score : f64 = 0.0f64;


        for word in self.all_words.iter() {
            if self.filtered(word) {
                continue;
            }
            let score = probability.score_word(&word.clone());
            if score > guessed_word_score {
                guessed_word_score = score;
                guessed_word = word.clone();
            }
        }

        guessed_word
    }

    fn filtered(self: &Self, word: &String) -> bool {
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

    pub fn remove_word(self: &mut Self, word: &String) {
        self.all_words.remove(word);
    }
}



#[cfg(test)]
mod statistics_tests {
    use crate::wordle::{LetterState};
    use super::*;

    #[test]
    fn default() {
        let result: Statistics = Default::default();
        assert_eq!(result.all_words.len(), 0);
        assert_eq!(result.filters.len(), 0);
    }

    #[test]
    fn add_word() {
        let mut result: Statistics = Default::default();
        result.add_word(&String::from("test"));
        assert_eq!(result.all_words.len(), 1);
        assert_eq!(result.all_words.contains(&String::from("test")), true);
    }

    fn default_word() -> Word {
        let mut word: Word = Default::default();
        word.letters[0].value = 'a';
        word.letters[0].set_state(LetterState::Present);

        word.letters[1].value = 'b';
        word.letters[1].set_state(LetterState::Present);

        word.letters[2].value = 'c';
        word.letters[2].set_state(LetterState::Present);

        word.letters[3].value = 'd';
        word.letters[3].set_state(LetterState::Present);

        word.letters[4].value = 'e';
        word.letters[4].set_state(LetterState::Present);

        word
    }

    #[test]
    fn guess_present() {
        let mut result: Statistics = Default::default();

        result.add_word(&String::from("abcde"));
        result.filters.push(default_word().clone());

        // The word is filtered out so the guess should be empty
        assert_eq!(result.guess(), "");

        // The word is not filtered out so the guess should be the word
        result.add_word(&String::from("edbca"));
        assert_eq!(result.guess(), "edbca");

        let mut filter = default_word().clone();
        filter.letters[0].set_state(LetterState::Incorrect);
        result.filters.push(filter.clone());

        // The word is filtered out so the guess should be empty
        assert_eq!(result.guess(), "");
    }

    #[test]
    fn clear() {
        let mut result: Statistics = Default::default();
        result.add_word(&String::from("test"));
        assert_eq!(result.all_words.len(), 1);
        result.clear();
        assert_eq!(result.all_words.len(), 0);
    }

    #[test]
    #[should_panic]
    fn add_word_panic() {
        let mut result: Statistics = Default::default();
        result.add_word(&String::from("testtest"));
    }
}
