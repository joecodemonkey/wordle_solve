#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::error::Error;
use eframe::egui;
use reqwest;

const WORD_LENGTH: usize = 5;
const MAX_ATTEMPTS: usize = 5;

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let mut app = WordleSolve::default();

    eframe::run_native(
        "Wordle Solver",
        options,
        Box::new(|_cc| {
            Box::<WordleSolve>::new(app)
        }),
    )
}

#[derive(Debug, Clone)]
enum WordleSquareState {
    Disabled,
    Incorrect,
    Correct,
    Present,
}

impl Default for WordleSquareState {
    fn default() -> Self {
        WordleSquareState::Disabled
    }
}

#[derive(Debug, Clone, Default)]
struct WordleSquare {
    // a char would be more efficient here for storage, but would
    // require conversion to string at runtime
    value: char,
    state: WordleSquareState,
}

#[derive(Debug, Clone)]
struct WordleWord {
    value: Vec<WordleSquare>,
}

impl Default for WordleWord {
    fn default() -> Self {
        Self {
            value: vec![WordleSquare::default(); WORD_LENGTH],
        }
    }
}

#[derive(Debug, Clone)]
struct WordleSolve {
    words_url: String,
    words: Vec<WordleWord>,
    guess: String,
    guess_num: usize,
    downloaded_words: Vec<String>,
    possible_words: Vec<usize>,
    statistics: WordleStatistics,
}

#[derive(Debug, Clone)]
struct WordleStatistics {
    letters: Vec<std::collections::HashMap<char, f64>>
}

impl WordleStatistics {
    fn new(words: &Vec<String>, possible_words: &Vec<usize>) ->WordleStatistics  {

         let mut ret = WordleStatistics {letters: Vec::new() };

        // reset us to 0

        for _ in 0..WORD_LENGTH {
            ret.letters.push(std::collections::HashMap::new());
        }

        // for every possible word
        for word_idx in possible_words {

            // snag the word from the word list
            let word = words.iter().nth(*word_idx).unwrap();

            // loop over the letters
            for letter_idx in 0..word.len() {
                // get the letter
                let val: char = word.chars().nth(letter_idx).unwrap();

                // increment the value by 1 if it exists or set it to 1 if it does not
                ret.letters[letter_idx].entry(val).and_modify(|counter| *counter += 1.0f64).or_insert(1.0f64);
            }
        }

        for stat in ret.letters.iter_mut() {
            let count = stat.iter().count() as f64;
            for (_, val) in stat.iter_mut() {
                *val /= count;
            }
        }
        ret
    }

    fn score(&self, word: &String) -> f64 {

        let mut score = 0.0;

        for (idx, letter) in word.chars().enumerate() {
            match self.letters.iter().nth(idx) {
                Some(map) => match map.get(&letter) {
                    Some(value) => score += value,
                    None => return 0.0f64 // if there is no value for letter at this pos, word is impossible
                }
                None => { }
            }
        }
        score
    }

}

impl Default for WordleSolve {
    fn default() -> Self {
        Self {
            words_url: "https://raw.githubusercontent.com/tabatkins/wordle-list/main/words".to_owned(),
            words: vec![WordleWord::default(); 6],
            guess: "".to_string(),
            guess_num: 0,
            downloaded_words: Vec::new(),
            possible_words: Vec::new(),
            statistics: WordleStatistics::new(&Vec::new(), &Vec::new()),
        }
    }
}
impl WordleSolve {
    fn download(&mut self) -> Result<(), Box<dyn Error>> {
        // Download the file content

        let mut response = reqwest::blocking::get(self.words_url.as_str())?;

        match response.text() {
            Ok(content) => {
                self.downloaded_words.clear();
                content.split("\n").for_each(|s| {
                    self.downloaded_words.push(s.to_string());
                });
                if self.downloaded_words.iter().count() > 0 {
                    self.guess_num = 0;
                    self.guess = "".to_string();
                    self.words = vec![WordleWord::default(); 6];
                    self.possible_words.clear();
                    self.guess();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        Ok(())
    }
    fn guess(&mut self) {
        self.filter();
        self.statistics = WordleStatistics::new(&self.downloaded_words, &self.possible_words);
        let mut max_score = 0.0f64;
        for word_idx in self.possible_words.iter() {
            let word = self.downloaded_words.iter().nth(*word_idx).unwrap();
            let score = self.statistics.score(word);
            if score > max_score {
                max_score = score;
                self.guess = word.clone();
            }
        }
        println!("max score is {}", max_score);
        println!("guess is {}", self.guess);
    }
    fn filter(&mut self) {
        if self.possible_words.is_empty() {
            for idx in 0..self.downloaded_words.iter().len() {
                self.possible_words.push(idx);
            }
        }
        if self.guess_num == 0 {
            return;
        }

        let mut possible_words: Vec<usize> = Vec::new();
        for (idx, word_idx) in self.possible_words.iter().enumerate() {
            if !self.filter_word(self.downloaded_words.get(*word_idx).unwrap()) {
                possible_words.push(*word_idx);
            }
        }
        std::mem::swap(&mut self.possible_words, &mut possible_words);
    }

    fn filter_word(&self, word: &String) -> bool {
        for guess in self.words.iter() {
            if Self::guess_filter_word(guess, word) {
                return true;
            }
        }
        false
    }

    fn guess_filter_word(guess: &WordleWord, word: &String) -> bool {
        for (index, letter) in guess.value.iter().enumerate() {
            if Self::guess_filter_letter(letter, word, index) {
                return true;
            }
        }
        false
    }

    fn guess_filter_letter(letter: &WordleSquare, word: &String, idx: usize) -> bool {

        match letter.state {
            WordleSquareState::Correct => {
                if letter.value != word.chars().nth(idx).unwrap() {
                    return true;
                }
            }
            WordleSquareState::Incorrect => {
                if letter.value == word.chars().nth(idx).unwrap() {
                    return true;
                }
                if word.contains(letter.value) {
                    return true;
                }
            }
            WordleSquareState::Present => {
                if !word.contains(letter.value) {
                    return true;
                }
            }
            _ => {}
        }

        false
    }
}

impl eframe::App for WordleSolve {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wordle Solver");
            ui.horizontal(|ui| {
                let name_label = ui.label("Word Source URL: ");
                ui.text_edit_singleline(&mut self.words_url)
                    .labelled_by(name_label.id);
                let download_button = egui::Button::new("↻");

                if ui.add(download_button).clicked() {
                    self.download();
                }
            });

            egui::Grid::new("wordle_squares").show(ui, |ui| {
                let mut row: usize = 0;
                while row < MAX_ATTEMPTS {
                        let mut col: usize = 0;
                        while col < WORD_LENGTH {
                            let mut value : String = self.words[row].value[col].value.to_string();
                            match self.words[row].value[col].state {
                                WordleSquareState::Disabled => {
                                    let button = egui::Button::new(" ");
                                    ui.add_enabled(false, button);
                                }
                                WordleSquareState::Incorrect => {
                                    let button = egui::Button::new(value);
                                    if ui.add(button).clicked() {
                                        self.words[row].value[col].state = WordleSquareState::Correct;
                                    }
                                }
                                WordleSquareState::Correct => {
                                    let button = egui::Button::new(value).fill(egui::Color32::GREEN);
                                    if ui.add(button).clicked() {
                                        self.words[row].value[col].state = WordleSquareState::Present;
                                    }
                                }
                                WordleSquareState::Present => {
                                    let button = egui::Button::new(value).fill(egui::Color32::YELLOW);
                                    if ui.add(button).clicked() {
                                        self.words[row].value[col].state = WordleSquareState::Incorrect;
                                    }
                                }
                            }
                            col += 1;
                        }
                        ui.end_row();
                    row += 1;
                }
            });
            let guess_button = egui::Button::new("Guess");
            if ui.add(guess_button).clicked() {
                if(self.guess_num < MAX_ATTEMPTS) {
                    self.guess_num += 1;
                    self.filter();
                    let mut word = self.words.iter_mut().nth(self.guess_num - 1).unwrap();
                    for (idx, letter) in self.guess.chars().enumerate() {
                        word.value[idx].value = letter.clone();
                        word.value[idx].state = WordleSquareState::Incorrect;
                    }
                }
            }
            let word_count = egui::Label::new("Word Count: ".to_string() + &self.downloaded_words.len().to_string());
            ui.add(word_count);
        });
    }
}