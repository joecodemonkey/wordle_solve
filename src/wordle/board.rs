use egui::Vec2;
use super::{Letter, Word};

#[derive(Debug, Clone)]
pub struct Board {
    words: Vec<Word>,
}

pub const MAX_LETTERS: usize = 5;
pub const MAX_ATTEMPTS: usize = 6;


impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            words: Vec::new(),
        };
        board.words.resize(MAX_ATTEMPTS, Word::new());
        for word in board.words.iter_mut() {
            word.resize(MAX_LETTERS, Default::default());
        }

        board
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    index: usize,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = &'a Word;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.board.words.len() {
            let result = &self.board.words[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl IntoIterator for Board {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Word;
    type IntoIter = std::slice::Iter<'a, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut Word;
    type IntoIter = std::slice::IterMut<'a, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.iter_mut()
    }
}

impl Board {
    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            board: self,
            index: 0,
        }
    }
}

impl Board {
    // Method to create a mutable iterator over the words
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Vec<Letter>> {
        self.words.iter_mut()
    }
}


#[cfg(test)]
mod letter_tests {
    use crate::wordle::LetterState;
    use super::*;

    #[test]
    fn default() {
        let result: Board = Default::default();
        assert_eq!(result.iter().count(), MAX_ATTEMPTS);
        for word in result.iter() {
            assert_eq!(word.iter().count(), MAX_LETTERS);
            for letter in word.iter() {
                assert_eq!(letter.value, ' ');
                assert_eq!(letter.get_state(), LetterState::Disabled);
            }
        }
    }
}