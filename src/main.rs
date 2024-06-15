#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::error::Error;
use eframe::egui;
use reqwest;
mod wordle;
use wordle::*;

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let app = WordleSolve::default();

    eframe::run_native(
        "Wordle Solver",
        options,
        Box::new(|_cc| {
            Box::<WordleSolve>::new(app)
        }),
    )
}

#[derive(Debug, Clone)]
struct WordleSolve {
    words_url: String,
    board: Board,
    guess: String,
    guess_num: usize,
    statistics: Statistics,
}


impl Default for WordleSolve {
    fn default() -> Self {
        Self {
            words_url: "https://raw.githubusercontent.com/tabatkins/wordle-list/main/words".to_owned(),
            board: Board::default(),
            guess: "".to_string(),
            guess_num: 0,
            statistics: Statistics::default(),
        }
    }
}
impl WordleSolve {
    fn download(&mut self) -> Result<(), Box<dyn Error>> {
        // Download the file content

        let response = reqwest::blocking::get(self.words_url.as_str())?;

        match response.text() {
            Ok(content) => {
                self.statistics.clear();
                content.split("\n").for_each(|s| {
                    self.statistics.add_word(&s.to_string().clone());
                });
                self.statistics.filters.clear();
                self.board = Board::default();
                self.guess = "".to_string();
                self.guess_num = 0;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        Ok(())
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
                    let _ = self.download();
                }
            });

            egui::Grid::new("wordle_squares").show(ui, |ui| {
                for mut row in self.board.iter_mut() {
                    for mut col in row.iter_mut() {
                        if ui.add(egui::Button::new(col.value.to_string()).fill(col.get_color())).clicked() {
                            col.toggle();
                        }
                    }
                    ui.end_row();
                }
            });
            let guess_button = egui::Button::new("Guess");
            if ui.add(guess_button).clicked() {
                if self.guess_num < MAX_ATTEMPTS {
                    self.guess_num += 1;
                    self.filter();
                    let word = self.board.iter_mut().nth(self.guess_num - 1).unwrap();
                    for (idx, letter) in self.guess.chars().enumerate() {
                        word[idx].value = letter.clone();
                        word[idx].set_state(LetterState::Incorrect);
                    }
                }
            }
            let word_count = egui::Label::new("Word Count: ".to_string() + &self.downloaded_words.len().to_string());
            ui.add(word_count);
        });
    }
}