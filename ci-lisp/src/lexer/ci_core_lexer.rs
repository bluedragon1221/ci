use crate::lexer::{CIBaseLexer, CIBaseLexerState, CILexerError, Lexer, SuperLexer, SuperLexerState, Token};

#[derive(Default)]
pub struct CICoreLexerState {
    base_state: CIBaseLexerState,
    
    in_string: bool,
}

impl CICoreLexerState {
    pub fn is_in_string(&self) -> bool {
        self.in_string
    }

    pub fn toggle_in_string(&mut self) {
        self.in_string = !self.in_string;
    }
}

impl SuperLexerState for CICoreLexerState {
    type SubState = CIBaseLexerState;

    fn sub(&mut self) -> &mut CIBaseLexerState {
        &mut self.base_state
    }
}

pub struct CICoreLexer {}

impl SuperLexer for CICoreLexer {
    type Sub = CIBaseLexer;
    type State = CICoreLexerState;

    fn handle_char(ch: char, state: &mut CICoreLexerState) -> Result<(), CILexerError> {
        match ch {
            ' ' => {
                if !state.is_in_string() {
                    state.sub().flush_word()
                } else {
                    state.sub().push_char(' ')
                };
            },
            '"' => {
                if !state.is_in_string() {
                    // beginning quote
                    state.sub().flush_word();
                    state.toggle_in_string();
                    state.sub().push_char('"');
                } else {
                    // ending quote
                    state.sub().push_char('"');
                    state.toggle_in_string();
                    state.sub().flush_word();
                };
            }
            '(' => {
                if !state.is_in_string() {
                    state.sub().flush_word();
                    state.sub().push_token(Token::LParen);
                } else {
                    state.sub().push_char('(');
                }
            },
            ')' => {
                if !state.is_in_string() {
                    state.sub().flush_word();
                    state.sub().push_token(Token::RParen);
                } else {
                    state.sub().push_char(')');
                }
            }
            a => Self::Sub::handle_char(a, state.sub())?
        }

        Ok(())
    }

    fn handle_eof(state: &mut Self::State) -> Result<(), CILexerError> {
        if state.is_in_string() {
            return Err(CILexerError::UnmatchedQuotes);
        }

        Ok(Self::Sub::handle_eof(state.sub())?)
    }
}
