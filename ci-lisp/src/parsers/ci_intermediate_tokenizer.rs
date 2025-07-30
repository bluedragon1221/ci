use std::cmp::Ordering;

use crate::{ast::{IntermediateToken, Token}, parser_types::{CIParserError, Parser, ParserState}};

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
    type Output = Vec<IntermediateToken>;

    fn take_tokens(self) -> Self::Output {
        self.new_tokens
    }
}

#[derive(Default)]
pub struct CIIntermediateTokenizer {}

impl CIIntermediateTokenizer {
    fn handle_token(token: Token, state: &mut CIIntermediateTokenizerState) -> Result<(), CIParserError> {
        match token {
            Token::LParen => {
                state.cur_paren_level += 1;
                state.push_token(IntermediateToken::LParen(state.cur_paren_level));
            },
            Token::LCurly => {
                state.cur_paren_level += 1;
                state.push_token(IntermediateToken::LCurly(state.cur_paren_level));
            },
            Token::LBracket => {
                state.cur_paren_level += 1;
                state.push_token(IntermediateToken::LBracket(state.cur_paren_level));
            }
            Token::Value(a) => state.push_token(IntermediateToken::Value(a)),
            Token::Hash => state.push_token(IntermediateToken::Hash),
            Token::RParen => {
                state.push_token(IntermediateToken::RParen(state.cur_paren_level));
                state.cur_paren_level -= 1;
            },
            Token::RCurly => {
                state.push_token(IntermediateToken::RCurly(state.cur_paren_level));
                state.cur_paren_level -= 1;
            },
            Token::RBracket => {
                state.push_token(IntermediateToken::RBracket(state.cur_paren_level));
                state.cur_paren_level -= 1;
            }
            Token::EOF => {
                state.push_token(IntermediateToken::EOF);

                // check paren levels
                match state.cur_paren_level.cmp(&0) {
                    Ordering::Less => return Err(CIParserError::MissingOpenParen(state.cur_paren_level)),
                    Ordering::Greater => return Err(CIParserError::MissingCloseParen(state.cur_paren_level)),
                    Ordering::Equal => ()
                }
            },
        };

        Ok(())
    }
}

impl Parser for CIIntermediateTokenizer {
    type Input = Vec<Token>;
    type Output = Vec<IntermediateToken>;

    fn parse(&self, tokens: Self::Input) -> Result<Self::Output, CIParserError> {
        let mut state = CIIntermediateTokenizerState::default();

        for i in tokens.into_iter() {
            Self::handle_token(i, &mut state)?;
        }

        Ok(state.take_tokens())        
    } 
}
