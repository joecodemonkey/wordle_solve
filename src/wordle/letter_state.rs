
    #[derive(Debug, Clone, PartialEq)]
    pub enum LetterState {
        Disabled,
        Incorrect,
        Correct,
        Present,
    }

    impl Default for LetterState {
        fn default() -> Self {
            LetterState::Disabled
        }
    }

    impl LetterState {
        pub fn toggle(&mut self) -> LetterState {
            *self = match self {
                LetterState::Disabled =>  LetterState::Disabled,
                LetterState::Incorrect => LetterState::Correct,
                LetterState::Correct => LetterState::Present,
                LetterState::Present => LetterState::Incorrect,
            };

            self.clone()
        }
    }

    #[cfg(test)]
    mod letter_state_tests {
        use super::*;

        #[test]
        fn default() {
            let result: LetterState = Default::default();
            assert_eq!(result, LetterState::Disabled);
        }

        #[test]
        fn toggle() {
            let mut result: LetterState = LetterState::Disabled;
            assert_eq!(result.toggle(), LetterState::Disabled);
            result = LetterState::Present;
            assert_eq!(result.toggle(), LetterState::Incorrect);
            assert_eq!(result.toggle(), LetterState::Correct);
            assert_eq!(result.toggle(), LetterState::Present);
            assert_eq!(result.toggle(), LetterState::Incorrect);
        }
    }