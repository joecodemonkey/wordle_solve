#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::error::Error;
use eframe::egui;
use reqwest;
use error_chain::{error_chain, stringify_internal};
use std::io::Read;
use egui::WidgetType::Label;

const WORD_LENGTH: usize = 5;
const MAX_ATTEMPTS: usize = 5;

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let mut app = WordleSolve::default();

    for mut word in app.words[0].value.iter_mut() {
        word.state = WordleSquareState::Editable;
    }

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
    Editable,
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
    value: String,
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
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        Ok(())
    }
}

trait Guess {
    fn guess(&mut self) -> Result<(), Box<dyn Error>>;

    fn filter(&mut self) -> Result<(), Box<dyn Error>>;

    fn filter_word(&self, word: &String) -> bool;

    fn guess_filter_word(guess: &WordleWord, word: &String) -> bool;

    fn guess_filter_letter(letter: &WordleSquare,  word: &String, idx: usize) -> bool;
}

impl Guess for WordleSolve {
    fn guess(&mut self) -> Result<(), Box<dyn Error>> {
        self.filter()?;
        Ok(())
    }
    fn filter(&mut self) -> Result<(), Box<dyn Error>> {
        self.possible_words.clear();

        // for each word we've downloaded
        for (index, word) in self.downloaded_words.iter ().enumerate() {
            if !self.filter_word(word) {
                self.possible_words.push(index);
            }
        }

        Ok(())
    }

    fn guess_filter_letter(letter: &WordleSquare, word: &String, idx: usize) -> bool {

        match letter.state {
            WordleSquareState::Correct => {
                if letter.value.chars().nth(0).unwrap() != word.chars().nth(idx).unwrap() {
                    return true;
                }
            }
            WordleSquareState::Incorrect => {
                if letter.value.chars().nth(0).unwrap() == word.chars().nth(idx).unwrap() {
                    return true;
                }
                if word.contains(letter.value.as_str()) {
                    return true;
                }
            }
            WordleSquareState::Present => {
                if !word.contains(letter.value.as_str()) {
                    return true;
                }
            }
            _ => {}
        }

        false
    }

    fn guess_filter_word(guess: &WordleWord, word: &String) -> bool {
        for (index, letter) in guess.value.iter().enumerate() {
            if(Self::guess_filter_letter(letter, word, index)) {
                return true;
            }
        }
        false
    }

    fn filter_word(&self, word: &String) -> bool {
        for guess in self.words.iter() {
            if Self::guess_filter_word(guess, word) {
                return true;
            }
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

                if(ui.add(download_button).clicked()) {

                    self.download().unwrap();

                    //update_words(self);
                }
            });

            egui::Grid::new("wordle_squares").show(ui, |ui| {
                let mut row: usize = 0;
                while row < MAX_ATTEMPTS {
                        let mut col: usize = 0;
                        while col < WORD_LENGTH {
                            if matches!(self.words[row].value[col].state, WordleSquareState::Editable) {
                                let text_edit = egui::TextEdit::singleline(&mut self.words[row].value[col].value)
                                    .char_limit(1);
                                ui.add(text_edit);
                            } else {
                                let text_edit = egui::TextEdit::singleline(&mut self.words[row].value[col].value)
                                    .char_limit(1).interactive(false);
                                ui.add(text_edit);
                            }
                            col += 1;
                        }
                        ui.end_row();
                    row += 1;
                }
                let guess_button = egui::Button::new("Guess");
                ui.add(guess_button);
                let word_count = egui::Label::new(("Word Count: ".to_string() + &self.downloaded_words.len().to_string()));
                ui.add(word_count);
            });
        });
    }
}