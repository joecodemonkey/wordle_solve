use egui::CursorIcon::Default;
use egui::Vec2;
use super::Word;

#[derive(Debug, Clone)]
pub struct Board {
    words: Vec<Word>,
}

pub const MAX_LETTERS: usize = 5;
pub const MAX_ATTEMPTS: usize = 6;


impl Default for Board {
    fn default() -> Self {
        let mut board = Default::default;
        board.words.resize(MAX_ATTEMPTS);
        for word in board.words.iter_mut() {
            word.resize(MAX_LETTERS);
        }

        board
    }
}