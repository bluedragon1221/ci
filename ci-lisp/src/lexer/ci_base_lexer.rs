use crate::lexer::{CILexerError, Lexer, LexerState, Token};

#[derive(Default)]
pub struct CIBaseLexerState {
    tokens: Vec<Token>,
    cur_word: String
}

impl LexerState for CIBaseLexerState {
    fn take_tokens(&mut self) -> Vec<Token> {
        std::mem::take(&mut self.tokens)
    }
}

impl CIBaseLexerState {
    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token)
    }

    pub fn push_char(&mut self, ch: char) {
        self.cur_word.push(ch)
    }

    pub fn clear_cur_word(&mut self) {
        self.cur_word.clear();
    }

    pub fn flush_word(&mut self) {
        if !self.cur_word.is_empty() {
            self.push_token(Token::guess_value(&self.cur_word));
            self.clear_cur_word();
        }
    }
}

#[derive(Default)]
pub struct CIBaseLexer {}

impl Lexer for CIBaseLexer {
    type Input = String;
    type State = CIBaseLexerState;
    
    fn handle_char(ch: char, state: &mut CIBaseLexerState) -> Result<(), CILexerError> {
        Ok(state.push_char(ch))
    }

    fn handle_eof(state: &mut CIBaseLexerState) -> Result<(), CILexerError> {
        Ok(state.push_token(Token::EOF))
    }
}
