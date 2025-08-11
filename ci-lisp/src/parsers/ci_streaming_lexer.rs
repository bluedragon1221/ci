use crate::{ast::Token, parser_types::{CIParserError, Parser}};

#[derive(Default)]
struct CIStreamingLexerState {
    tokens: Vec<Token>,
    cur_word: String,

    in_string: bool
}

impl CIStreamingLexerState {
    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token)
    }

    pub fn push_char(&mut self, ch: char) {
        self.cur_word.push(ch)
    }

    pub fn toggle_in_string(&mut self) {
        self.in_string = !self.in_string;
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

    pub fn take_tokens(self) -> Vec<Token> {
        self.tokens
    }
}

#[derive(Default)]
pub struct CIStreamingLexer {}

impl CIStreamingLexer {
    fn handle_char(ch: char, state: &mut CIStreamingLexerState) {
        match ch {
            ' ' | '\n' | '\t' if !state.in_string => {
                state.flush_word()
            },
            '"' => {
                if !state.in_string {
                    // beginning quote
                    state.flush_word();
                    state.toggle_in_string();
                    state.push_char('"');
                } else {
                    // ending quote
                    state.push_char('"');
                    state.toggle_in_string();
                    state.flush_word();
                };
            }
            '(' if !state.in_string => {
                state.flush_word();
                state.push_token(Token::LParen);
            }
            ')' if !state.in_string => {
                state.flush_word();
                state.push_token(Token::RParen);
            }
            '{' if !state.in_string => {
                state.flush_word();
                state.push_token(Token::LCurly);
            }
            '}' if !state.in_string => {
                state.flush_word();
                state.push_token(Token::RCurly);
            }
            '[' if !state.in_string => {
                state.flush_word();
                state.push_token(Token::LBracket);
            }
            ']' if !state.in_string => {
                state.flush_word();
                state.push_token(Token::RBracket);
            }
            '#' if !state.in_string => {
                state.flush_word();
                state.push_token(Token::Hash);
            }
            a => state.push_char(a)
        }
    }
}

impl Parser for CIStreamingLexer {
    type Input = String;
    type Output = Vec<Token>;

    fn parse(&self, tokens: String) -> Result<Vec<Token>, CIParserError> {
        let mut state = CIStreamingLexerState::default();

        for i in tokens.chars() {
            Self::handle_char(i, &mut state);
        }

        if state.in_string {
            Self::handle_char('"', &mut state);
        } else {
            state.flush_word();
        }

        state.push_token(Token::EOF);

        Ok(state.take_tokens())
    }
}
