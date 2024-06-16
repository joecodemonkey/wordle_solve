use std::default::Default;
use crate::wordle::{LetterState, MAX_LETTERS, Word};

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
        self.word_count += 1;
    }

    fn add_letter(self: &mut Self, letter: &char, index: usize) {
        let map = self.counts.iter_mut().nth(index).unwrap();
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
                    let num : f64 = *value as f64;
                    ret += num / self.word_count as f64;
                }
                None => { return 0.0f64; }
            }
        }
        ret
    }
}


#[cfg(test)]
mod letter_probability_tests {
    use super::*;

    fn float_compare(a: f64, b: f64, digits: usize) -> bool {
        let a_int = (a * 10f64.powi(digits as i32)).round() as i64;
        let b_int = (b * 10f64.powi(digits as i32)).round() as i64;
        a_int == b_int
    }

    #[test]
    fn default() {
        let result: LetterProbability = Default::default();
        assert_eq!(result.counts.len(), MAX_LETTERS);
        assert_eq!(result.word_count, 0);
    }
    #[test]
    fn add_word() {
        let mut result: LetterProbability = Default::default();
        result.add_word(&String::from("hello"));
        assert_eq!(result.word_count, 1);
        assert_eq!(result.counts[0].get(&'h').unwrap(), &1u64);
        assert_eq!(result.counts[0].get(&'e'), None);

        assert_eq!(result.counts[1].get(&'e').unwrap(), &1u64);
        assert_eq!(result.counts[1].get(&'k'), None);

        assert_eq!(result.counts[2].get(&'l').unwrap(), &1u64);
        assert_eq!(result.counts[2].get(&'z'), None);

        assert_eq!(result.counts[3].get(&'l').unwrap(), &1u64);
        assert_eq!(result.counts[3].get(&'z'), None);

        assert_eq!(result.counts[4].get(&'o').unwrap(), &1u64);
        assert_eq!(result.counts[4].get(&'z'), None);
    }

    #[test]
    fn score_word() {
        let mut result: LetterProbability = Default::default();
        result.add_word(&String::from("hello"));
        result.add_word(&String::from("world"));
        assert!(float_compare(result.score_word(&String::from("hello")), 3.0f64, 3));
        assert_eq!(result.score_word(&String::from("world")), 3.0f64);
        assert_eq!(result.score_word(&String::from("horld")), 3.0f64);
        result.add_word(&String::from("weird"));
        assert!(float_compare(result.score_word(&String::from("hello")), 2.333f64, 3));
        assert!(float_compare(result.score_word(&String::from("world")), 2.6666f64, 3));
        assert!(float_compare(result.score_word(&String::from("horld")) , 2.3333f64, 3));

        assert_eq!(result.score_word(&String::from("rends")), 0.0f64);
    }
}
