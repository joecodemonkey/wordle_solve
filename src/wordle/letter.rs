use super::LetterState;

#[derive(Debug, Clone)]
pub struct Letter {
    // a char would be more efficient here for storage, but would
    // require conversion to string at runtime
    pub value: char,
    state: LetterState,
    fill_color: egui::Color32,
    text_color: egui::Color32,
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            value: ' ',
            state: Default::default(),
            fill_color: egui::Color32::BLACK,
            text_color: egui::Color32::WHITE,
        }
    }
}

impl Letter {
    pub fn toggle(&mut self) {
        self.state.toggle();
        self.set_color();
    }

    fn set_color(&mut self) {
        self.fill_color = match self.state {
            LetterState::Disabled => egui::Color32::BLACK,
            LetterState::Present => egui::Color32::YELLOW,
            LetterState::Incorrect => egui::Color32::BLACK,
            LetterState::Correct => egui::Color32::LIGHT_GREEN,
        };

        self.text_color = match self.state {
            LetterState::Disabled => egui::Color32::WHITE,
            LetterState::Present => egui::Color32::BLACK,
            LetterState::Incorrect => egui::Color32::WHITE,
            LetterState::Correct => egui::Color32::BLACK,
        };
    }

    pub fn set_state(&mut self, state: LetterState) {
        self.state = state;
        self.set_color();
    }
    pub fn get_state(&self) -> LetterState {
        self.state.clone()
    }

    pub fn get_fill_color(&self) -> egui::Color32 {
        self.fill_color
    }

    pub fn get_text_color(&self) -> egui::Color32 {
        self.text_color
    }
}

#[cfg(test)]
mod letter_tests {
    use super::*;

    #[test]
    fn default() {
        let result: Letter = Letter::default();
        assert_eq!(result.value, ' ');
        assert_eq!(result.text_color, egui::Color32::WHITE);
        assert_eq!(result.state, LetterState::Disabled);
        assert_eq!(result.fill_color, egui::Color32::BLACK);
    }

    #[test]
    fn set_state() {
        let mut result: Letter = Letter::default();
        result.set_state(LetterState::Correct);
        assert_eq!(result.get_state(), LetterState::Correct);
        assert_eq!(result.get_fill_color(), egui::Color32::LIGHT_GREEN);
        assert_eq!(result.get_text_color(), egui::Color32::BLACK);

        result.set_state(LetterState::Incorrect);
        assert_eq!(result.get_state(), LetterState::Incorrect);
        assert_eq!(result.get_fill_color(), egui::Color32::BLACK);
        assert_eq!(result.get_text_color(), egui::Color32::WHITE);

        result.set_state(LetterState::Disabled);
        assert_eq!(result.get_state(), LetterState::Disabled);
        assert_eq!(result.get_fill_color(), egui::Color32::BLACK);
        assert_eq!(result.get_text_color(), egui::Color32::WHITE);


        result.set_state(LetterState::Present);
        assert_eq!(result.get_state(), LetterState::Present);
        assert_eq!(result.get_fill_color(), egui::Color32::YELLOW);
        assert_eq!(result.get_text_color(), egui::Color32::BLACK);
    }
}