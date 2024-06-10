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
    possible_words: Vec<usize>
}

#[derive(Debug, Clone)]
struct Wordletatistics {
    letter_statistics: Vec<std::collections::HashMap<char, f64>>
}

trait BuildStatistics {
    fn build_statistics(&mut self, words: &Vec<String>, possible_words: Vec<usize>) ;
}

impl BuildStatistics for Wordletatistics {
    fn build_statistics(&mut self, words: &Vec<String>, possible_words: Vec<usize>)  {

        // reset us to 0
        self.letter_statistics = Vec::new();
        for _ in 0..WORD_LENGTH {
            self.letter_statistics.push(std::collections::HashMap::new());
        }

        // for every possible word
        for word_idx in possible_words {

            // snag the word from the word list
            let word = words.iter().nth(word_idx).unwrap();

            // loop over the letters
            for letter_idx in 0..word.len() {
                // get the letter
                let val: char = word.chars().nth(letter_idx).unwrap();

                // increment the value by 1 if it exists or set it to 1 if it does not
                self.letter_statistics[letter_idx].entry(val).and_modify(|counter| *counter += 1.0f64).or_insert(1.0f64);
            }
        }

        for stat in self.letter_statistics.iter_mut() {
            let count = stat.iter().count() as f64;
            for (_, val) in stat.iter_mut() {
                *val /= count;
            }
        }
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
            possible_words: Vec::new()
        }
    }
}

trait Download {
    fn download(&mut self) -> Result<(), Box<dyn Error>>;
}
impl Download for WordleSolve {
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
}

trait Guess {
    fn guess(&mut self);

    fn filter(&mut self);

    fn filter_word(&self, word: &String) -> bool;

    fn guess_filter_word(guess: &WordleWord, word: &String) -> bool;

    fn guess_filter_letter(letter: &WordleSquare,  word: &String, idx: usize) -> bool;
}

impl Guess for WordleSolve {
    fn guess(&mut self) {
        self.filter();
    }
    fn filter(&mut self) {
        if self.possible_words.is_empty() {
            for idx in 0..self.downloaded_words.iter().len() {
                self.possible_words.push(idx);
            }
        }
        let mut possible_words: Vec<usize> = Vec::new();
        for (idx, word_idx) in self.possible_words.iter().enumerate() {
            if self.filter_word(self.downloaded_words.get(*word_idx).unwrap()) {
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
            ui.add(guess_button);
            let word_count = egui::Label::new("Word Count: ".to_string() + &self.downloaded_words.len().to_string());
            ui.add(word_count);
        });
    }
}