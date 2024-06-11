
    #[derive(Debug, Clone, PartialEq)]
    pub enum SquareState {
        Disabled,
        Incorrect,
        Correct,
        Present,
    }

    impl Default for SquareState {
        fn default() -> Self {
            SquareState::Disabled
        }
    }

    impl SquareState {
        pub fn toggle(&mut self) -> SquareState {
            *self = match self {
                SquareState::Disabled => SquareState::Incorrect,
                SquareState::Incorrect => SquareState::Correct,
                SquareState::Correct => SquareState::Present,
                SquareState::Present => SquareState::Incorrect,
            };

            self.clone()
        }
    }

    #[cfg(test)]
    mod squarestate_tests {
        use super::*;

        #[test]
        fn initialization_works() {
            let result: SquareState = Default::default();
            assert_eq!(result, SquareState::Disabled);
        }

        #[test]
        fn toggle() {
            let mut result: SquareState = SquareState::Disabled;

            assert_eq!(result.toggle(), SquareState::Incorrect);
            assert_eq!(result.toggle(), SquareState::Correct);
            assert_eq!(result.toggle(), SquareState::Present);
            assert_eq!(result.toggle(), SquareState::Incorrect);
        }
    }


