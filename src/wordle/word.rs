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
    pub fn filter(self: &Self, str: &String) -> bool {
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

                    for( idx, c) in str.chars().enumerate() {
                        if c == letter.value {
                            if self.letters[idx].get_state() != LetterState::Correct {
                                return true;
                            }
                        }
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

    pub fn set(self: &mut Self, word: &String) {
        if word.len() > MAX_LETTERS {
            panic!("Word {} of length {} is greater than allowed length {}", word, word.len(), MAX_LETTERS)
        }
        for (index, letter) in word.chars().enumerate() {
            self.letters[index].value = letter;
            self.letters[index].set_state(LetterState::Incorrect);
        }
    }
}

#[cfg(test)]
mod word_tests {
    use super::*;

    #[test]
    fn default() {
        let result: Word = Default::default();
        assert_eq!(result.letters.len(), MAX_LETTERS);
    }

    #[test]
    fn filter() {
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

        let str = String::from("abcde");
        assert_eq!(word.filter(&str), false);

        let str = String::from("abcdf");
        assert_eq!(word.filter(&str), true);

        let str = String::from("abcde");
        word.letters[4].set_state(LetterState::Correct);
        assert_eq!(word.filter(&str), false);

        word.letters[4].set_state(LetterState::Incorrect);
        assert_eq!(word.filter(&str), true);

        let str = String::from("aaaae");
        word.letters[0].set_state(LetterState::Incorrect);
        assert_eq!(word.filter(&str), true);
    }

    #[test]
    #[should_panic]
    fn filter_panic() {
        let word: Word = Default::default();
        let str = String::from("abcde");
        word.filter(&str);
    }
}