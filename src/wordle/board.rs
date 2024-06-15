use std::default::Default;
use super::{MAX_LETTERS, Word};

#[derive(Debug, Clone)]
pub struct Board {
    pub words: Vec<Word>,
}

pub const MAX_ATTEMPTS: usize = 6;

impl Default for Board {
    fn default() -> Self {
        Board {
            words: vec![Default::default(); MAX_ATTEMPTS],
        }
    }
}

#[cfg(test)]
mod letter_tests {
    use crate::wordle::LetterState;
    use super::*;

    #[test]
    fn default() {
        let result: Board = Default::default();
        assert_eq!(result.words.iter().count(), MAX_ATTEMPTS);
        for word in result.words.iter() {
            assert_eq!(word.letters.iter().count(), MAX_LETTERS);
            for letter in word.letters.iter() {
                assert_eq!(letter.value, ' ');
                assert_eq!(letter.get_state(), LetterState::Disabled);
            }
        }
    }
}

