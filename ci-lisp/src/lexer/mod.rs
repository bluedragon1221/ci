mod ci_base_lexer;
pub use ci_base_lexer::{CIBaseLexer, CIBaseLexerState};

mod ci_core_lexer;
pub use ci_core_lexer::{CICoreLexer, CICoreLexerState};

mod ci_sugar_lexer;
pub use ci_sugar_lexer::{CISugarLexer, CISugarLexerState};

mod token;
pub use token::{Token, Value};

#[derive(Debug, thiserror::Error)]
pub enum CILexerError {
    #[error("Unmatched quotes")]
    UnmatchedQuotes,
}

pub trait LexerInput {
    type Item;
    fn lexer_input_iter(&self) -> impl Iterator<Item = Self::Item>;
}

impl LexerInput for String {
    type Item = char;

    fn lexer_input_iter(&self) -> impl Iterator<Item = char> {
        self.chars()
    }
}

pub trait LexerToken {
    fn guess_value(word: &str) -> Self;
}

pub trait LexerState {
    fn take_tokens(&mut self) -> Vec<Token>;
}

// implement LexerState for any type that implements Deref, and it's Deref::Target implements LexerState
// impl<T> LexerState for T
// where
//     T: DerefMut,
//     T::Target: LexerState
// {
//     fn take_tokens(&mut self) -> Vec<Token> {
//         DerefMut::deref_mut(self).take_tokens()
//     }
// }

pub trait SuperLexerState {
    type SubState: LexerState + Default;

    fn sub(&mut self) -> &mut Self::SubState;
}

impl<T: SuperLexerState> LexerState for T {
    fn take_tokens(&mut self) -> Vec<Token> {
        T::SubState::take_tokens(self.sub())
    }
}

pub trait Lexer {
    type Input: LexerInput;
    type State: LexerState + Default;

    fn handle_char(ch: <Self::Input as LexerInput>::Item, state: &mut Self::State) -> Result<(), CILexerError>;
    fn handle_eof(state: &mut Self::State) -> Result<(), CILexerError>;

    fn tokenize(&self, input: Self::Input) -> Result<Vec<Token>, CILexerError> {
        let mut state = Self::State::default();

        for i in input.lexer_input_iter() {
            Self::handle_char(i, &mut state)?;
        }

        Self::handle_eof(&mut state)?;

        Ok(state.take_tokens())
    }
}

pub trait SuperLexer {
    type Sub: Lexer;
    type State: SuperLexerState<SubState = <Self::Sub as Lexer>::State> + Default;

    fn handle_char(ch: <<Self::Sub as Lexer>::Input as LexerInput>::Item, state: &mut Self::State) -> Result<(), CILexerError>;
    fn handle_eof(state: &mut Self::State) -> Result<(), CILexerError> {
        <Self::Sub as Lexer>::handle_eof(state.sub())
    }
}

impl<T: SuperLexer> Lexer for T {
    type Input = <T::Sub as Lexer>::Input;
    type State = T::State;

    fn handle_char(ch: <Self::Input as LexerInput>::Item, state: &mut Self::State) -> Result<(), CILexerError> {
        T::handle_char(ch, state)
    }
    fn handle_eof(state: &mut Self::State) -> Result<(), CILexerError> {
        T::handle_eof(state)
    }
}

// impl<T> SuperLexerState for T::State
// where
//     T: SuperLexer,
//     T::State: SuperLexer
// {
//     type SubState = T::State;
// }
