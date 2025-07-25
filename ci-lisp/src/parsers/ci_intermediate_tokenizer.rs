use std::cmp::Ordering;

use crate::{ast::{IntermediateToken, Token}, parser_types::{CIParserError, ParserState, SingleParserDefault}};

#[derive(Default)]
pub struct CIIntermediateTokenizerState {
    new_tokens: Vec<IntermediateToken>,
    cur_paren_level: i32,
}

impl CIIntermediateTokenizerState {
    fn push_token(&mut self, token: IntermediateToken) {
        self.new_tokens.push(token)
    }
}

impl ParserState for CIIntermediateTokenizerState {
    type OutputNode = IntermediateToken;

    fn take_tokens(self) -> Vec<Self::OutputNode> {
        self.new_tokens
    }
}

#[derive(Default)]
pub struct CIIntermediateTokenizer {}

impl SingleParserDefault for CIIntermediateTokenizer {
    type InputNode = Token;
    type OutputNode = IntermediateToken;
    type State = CIIntermediateTokenizerState;

    fn handle_token(token: Self::InputNode, state: &mut CIIntermediateTokenizerState) -> Result<(), CIParserError> {
        match token {
            Token::LParen => {
                state.cur_paren_level += 1;
                state.push_token(IntermediateToken::LParen(state.cur_paren_level));
            },
            Token::Value(a) => state.push_token(IntermediateToken::Value(a)),
            Token::RParen => {
                state.push_token(IntermediateToken::RParen(state.cur_paren_level));
                state.cur_paren_level -= 1;
            },
            Token::Fn => state.push_token(IntermediateToken::Fn),
            Token::EOF => {
                state.push_token(IntermediateToken::EOF);

                // check paren levels
                match state.cur_paren_level.cmp(&0) {
                    Ordering::Less => return Err(CIParserError::MissingOpenParen(state.cur_paren_level)),
                    Ordering::Greater => return Err(CIParserError::MissingCloseParen(state.cur_paren_level)),
                    Ordering::Equal => ()
                }
            },
            a => return Err(CIParserError::UnknownToken(Box::new(a)))
        };

        Ok(())
    }
}
