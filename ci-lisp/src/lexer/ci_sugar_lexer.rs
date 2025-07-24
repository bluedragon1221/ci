// use derive_more::{Deref, DerefMut};

use crate::lexer::{CICoreLexer, CICoreLexerState, CILexerError, Lexer, SuperLexer, SuperLexerState, Token};

#[derive(Default)]
pub struct CISugarLexerState {
    // #[deref]
    // #[deref_mut]
    core_state: CICoreLexerState,
}

impl SuperLexerState for CISugarLexerState {
    type SubState = CICoreLexerState;

    fn sub(&mut self) -> &mut CICoreLexerState {
        &mut self.core_state
    }
}

#[derive(Default)]
pub struct CISugarLexer {}

impl SuperLexer for CISugarLexer {
    type Sub = CICoreLexer;
    type State = CISugarLexerState;

    fn handle_char(ch: char, state: &mut CISugarLexerState) -> Result<(), CILexerError> {
        match ch {
            '{' if !state.sub().is_in_string() => {
                state.sub().sub().flush_word();
                state.sub().sub().push_token(Token::LCurly);
            }
            '}' if !state.sub().is_in_string() => {
                state.sub().sub().flush_word();
                state.sub().sub().push_token(Token::RCurly);
            }
            '[' if !state.sub().is_in_string() => {
                state.sub().sub().flush_word();
                state.sub().sub().push_token(Token::LBracket);
            }
            ']' if !state.sub().is_in_string() => {
                state.sub().sub().flush_word();
                state.sub().sub().push_token(Token::RBracket);
            }
            a => <CICoreLexer as Lexer>::handle_char(a, &mut state.core_state)?
        }

        Ok(())
    }
}
