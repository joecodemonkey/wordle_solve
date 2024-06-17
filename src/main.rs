#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)]

use std::error::Error;
use eframe::{egui};
use reqwest;

mod wordle;

use wordle::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]).with_resizable(false),

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
        let mut reset = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(40.0, eframe::epaint::FontFamily::Monospace),
            );

            ui.style_mut().text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::new(32.0, eframe::epaint::FontFamily::Monospace),
            );

            ui.style_mut().text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(20.0, eframe::epaint::FontFamily::Monospace),
            );

            egui::Grid::new("wordle_squares").show(ui, |ui| {
                for (idx, row) in self.board.words.iter_mut().enumerate() {
                    for col in row.letters.iter_mut() {
                        let button_text = egui::RichText::new(col.value.to_string())
                            .color(col.get_text_color());

                        let button = egui::Button::new(button_text).
                            fill(col.get_fill_color());

                        if ui.add(button).clicked() {
                            col.toggle();
                        }
                    }
                    if self.guess_num == idx + 1 {
                        let button_text = egui::RichText::new("🚫");

                        let button = egui::Button::new(button_text);

                        if ui.add(button).on_hover_text("Reject this guess").clicked() {
                         reset = true;
                        }
                    }
                    ui.end_row();

                }
            });
            ui.horizontal(|ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(20.0, eframe::epaint::FontFamily::Monospace),
                );

                let download_button = egui::Button::new("Download");

                if ui.add(download_button).on_hover_text("Download Words / Restart").clicked() {
                    let _ = self.download();
                }

                let guess_button = egui::Button::new("Guess");
                if ui.add(guess_button).on_hover_text("Guess the next word").clicked() && (self.guess != "" || self.guess_num == 0) {
                    self.statistics.filters.clear();
                    for word in self.board.words.iter() {
                        if word.letters.iter().all(|letter| letter.get_state() == LetterState::Disabled) {
                            continue;
                        }
                        self.statistics.filters.push(word.clone());
                    }
                    if self.guess_num < MAX_ATTEMPTS {
                        self.guess_num += 1;
                        self.guess = self.statistics.guess();
                        self.board.set_word(self.guess_num - 1, &self.guess);
                        let word = self.board.words.iter_mut().nth(self.guess_num - 1).unwrap();
                        for (idx, letter) in self.guess.chars().enumerate() {
                            word.letters[idx].value = letter.clone();
                            word.letters[idx].set_state(LetterState::Incorrect);
                        }
                    }
                }
            });
            let word_count = egui::Label::new("Words in Dictionary: ".to_string() + &self.statistics.len().to_string());
            ui.add(word_count);
            let mut end_text = "";
            if self.guess == "" && self.guess_num > 0 {
                end_text = if self.statistics.len() == 0 {
                    "No valid guesses left, Download Words First"
                } else {
                    "No valid guesses left"
                };
            }
            let button_text = egui::RichText::new(end_text)
                .color(egui::Color32::RED);
            let guess_label = egui::Label::new(button_text);
            ui.add(guess_label);

            ui.horizontal(|ui| {
                let url_label = egui::Label::new("Dictionary Source URL");

                ui.add(url_label);

                let mut text_edit = egui::TextEdit::singleline(&mut self.words_url).desired_width(400.0);

                ui.add(text_edit);
            });
            if reset {
                self.statistics.remove_word(&self.guess);
                self.guess = self.statistics.guess();
                self.board.set_word(self.guess_num - 1, &self.guess);
            }
        });
    }
}