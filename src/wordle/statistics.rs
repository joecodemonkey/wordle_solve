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
}



#[cfg(test)]
mod statistics_tests {
    use crate::wordle::LetterState;
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

    #[test]
    fn guess() {
        let mut result: Statistics = Default::default();
        result.add_word(&String::from("tests"));
        result.add_word(&String::from("tesas"));
        result.add_word(&String::from("tists"));
        result.add_word(&String::from("tescs"));
        result.add_word(&String::from("tasds"));

        let guess = result.guess();
        assert_eq!(guess, "tests");

        let mut filter = Word::default();
        filter.letters[0].value = 't';
        filter.letters[0].set_state(LetterState::Present);
        filter.letters[1].value = 'e';
        filter.letters[1].set_state(LetterState::Incorrect);
        filter.letters[2].value = 's';
        filter.letters[2].set_state(LetterState::Present);
        filter.letters[3].value = 't';
        filter.letters[3].set_state(LetterState::Present);
        filter.letters[4].value = 's';
        filter.letters[4].set_state(LetterState::Present);

        result.filters.push(filter);

        let guess = result.guess();
        assert_eq!(guess, "tasds");

        let mut filter = Word::default();
        filter.letters[0].value = 't';
        filter.letters[0].set_state(LetterState::Present);
        filter.letters[1].value = 'a';
        filter.letters[1].set_state(LetterState::Incorrect);
        filter.letters[2].value = 's';
        filter.letters[2].set_state(LetterState::Present);
        filter.letters[3].value = 't';
        filter.letters[3].set_state(LetterState::Present);
        filter.letters[4].value = 's';
        filter.letters[4].set_state(LetterState::Present);

        result.filters.push(filter);

        let guess = result.guess();
        assert_eq!(guess, "tists");
    }
}
