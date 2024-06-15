use super::{Letter, LetterState};

pub const MAX_LETTERS: usize = 5;

#[derive(Debug, Clone)]
pub struct Word {
  pub letters: Vec<Letter>,
}

impl Default for Word {
    fn default() -> Self {
        Self {
            letters: vec![Default::default(); MAX_LETTERS],
        }
    }
}

impl Word {
    pub fn filter(self: &mut Self, str: &String) -> bool {
        for (letter_idx, letter) in self.letters.iter().enumerate() {
            match letter.get_state() {
                LetterState::Disabled => {
                    panic!("Got Disabled Letter State for letter in word that shouldn't be disabled?!?");
                },
                LetterState::Present => {
                    str.chars().any(|c| c == letter.value);
                    if !str.contains(&letter.value.to_string()) {
                        return true;
                    }
                },
                LetterState::Incorrect => {
                    if str.chars().nth(letter_idx).unwrap() == letter.value {
                        return true;
                    }
                },
                LetterState::Correct => {
                    if str.chars().nth(letter_idx).unwrap() != letter.value {
                        return true;
                    }
                },
            }
        }
        false
    }
}