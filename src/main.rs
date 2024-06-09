#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

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
}

impl Default for WordleSolve {
    fn default() -> Self {
        Self {
            words_url: "https://raw.githubusercontent.com/tabatkins/wordle-list/main/words".to_owned(),
            words: vec![WordleWord::default(); 6],
            guess: "".to_string(),
            guess_num: 0
        }
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
                ui.add(download_button);
            });
            egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.vertical(|ui| {
                let mut row: usize = 0;
                while row < MAX_ATTEMPTS {
                    ui.horizontal(|ui| {
                        let mut col: usize = 0;
                        while col < WORD_LENGTH {
                            if matches!(self.words[row].value[col].state, WordleSquareState::Editable) {
                                let text_edit = egui::TextEdit::singleline(&mut self.words[row].value[col].value)
                                    .char_limit(1).desired_width(5.0f32);
                                ui.add(text_edit);
                            } else {
                                let text_edit = egui::TextEdit::singleline(&mut self.words[row].value[col].value)
                                    .char_limit(1).desired_width(5.0f32).interactive(false);
                                ui.add(text_edit);
                            }
                            col += 1;
                        }
                        ui.end_row();
                    });
                    row += 1;
                }
                let guess_button = egui::Button::new("Guess");
                ui.add(guess_button);
            });
            });
        });
    }
}