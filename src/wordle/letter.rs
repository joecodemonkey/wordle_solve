use super::LetterState;

#[derive(Debug, Clone)]
pub struct Letter {
    // a char would be more efficient here for storage, but would
    // require conversion to string at runtime
    pub value: char,
    pub state: LetterState,
}

impl Default for Letter {
    fn default() -> Self {
        Self {
            value: ' ',
            state: Default::default()
        }
    }
}

impl Letter {
    pub fn toggle(&mut self) {
        self.state.toggle();
    }
}

#[cfg(test)]
mod letter_tests {
    use super::*;

    #[test]
    fn default() {
        let result: Letter = Default::default();
        assert_eq!(result.value, ' ');
    }
}