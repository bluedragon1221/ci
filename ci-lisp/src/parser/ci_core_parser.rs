// self-made parser idea:
//   - [X] get the level of each parenthesis
//   - [X] find highest_level
//   - [X] get what's between first LParen(highest_value) and RParen(highest_value) 
//   - [X] replace that with ast
//   - [X] loop until highest_level is 0 (no parens)
//   time for the bugs!

use std::cmp::Ordering;

use crate::{
    lexer::{Token, Value},
    parser::{AstNode, CIParserError, Parser, ParserState, SeqParsers, SingleParser, SingleParserDefault}
};

#[derive(Debug)]
pub enum IntermediateToken {
    LParen(i32),
    Value(Value),
    RParen(i32),
    EOF,
    AstNode(AstNode),
}

#[derive(Default)]
pub struct IntermediateTokenizerState {
    new_tokens: Vec<IntermediateToken>,
    cur_paren_level: i32,
}

impl IntermediateTokenizerState {
    fn push_token(&mut self, token: IntermediateToken) {
        self.new_tokens.push(token)
    }
}

impl ParserState for IntermediateTokenizerState {
    type OutputNode = IntermediateToken;

    fn take_tokens(self) -> Vec<Self::OutputNode> {
        self.new_tokens
    }
}

#[derive(Default)]
pub struct IntermediateTokenizer {}

impl SingleParserDefault for IntermediateTokenizer {
    type InputNode = Token;
    type OutputNode = IntermediateToken;
    type State = IntermediateTokenizerState;

    fn handle_token(token: Self::InputNode, state: &mut IntermediateTokenizerState) -> Result<(), CIParserError> {
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

pub struct ParserStepState {
    new_tokens: Vec<IntermediateToken>,
    cur_node: Vec<AstNode>,

    highest_level: i32,
    in_highest_level: bool,
}

impl ParserStepState {
    pub fn new(tokens: &Vec<IntermediateToken>) -> Self {
        Self {
            new_tokens: Vec::new(),
            cur_node: Vec::new(),
            highest_level: Self::get_highest_level(tokens),
            in_highest_level: false
        }
    }

    fn push_token(&mut self, token: IntermediateToken) {
        self.new_tokens.push(token)
    }

    fn push_cur_node(&mut self, a: AstNode) {
        self.cur_node.push(a)
    }

    fn clear_cur_node(&mut self) {
        self.cur_node = Vec::new()
    }

    fn flush_cur_node(&mut self) -> Result<(), CIParserError> {
        let token = match self.cur_node.len() {
            0 => IntermediateToken::AstNode(AstNode::Value(Value::Nil)),
            1 => IntermediateToken::AstNode(self.cur_node[0].clone()),
            2 => IntermediateToken::AstNode(AstNode::Par {
                car: Box::new(self.cur_node[0].clone()),
                cdr: Box::new(self.cur_node[1].clone())
            }),
            _ => return Err(CIParserError::NodeFull(self.cur_node.len()))
        };

        self.new_tokens.push(token);
        self.clear_cur_node();

        Ok(())
    }

    fn get_highest_level(tokens: &Vec<IntermediateToken>) -> i32 {
        tokens.iter()
            .filter_map(|x| match x {
                IntermediateToken::LParen(n) => Some(*n),
                IntermediateToken::RParen(n) => Some(*n),
                _ => None
            })
            .max()
            .unwrap_or(0)
    }
}

impl ParserState for ParserStepState {
    type OutputNode = IntermediateToken;
    
    fn take_tokens(self) -> Vec<Self::OutputNode> {
        self.new_tokens
    }
}

#[derive(Default)]
pub struct ParserStep {}

impl SingleParser for ParserStep {
    type InputNode = IntermediateToken;
    type OutputNode = IntermediateToken;
    type State = ParserStepState;

    fn init_state(tokens: &Vec<Self::InputNode>) -> Self::State {
        ParserStepState::new(&tokens)
    }

    fn handle_token(token: IntermediateToken, state: &mut ParserStepState) -> Result<(), CIParserError> {
        match token {
            IntermediateToken::LParen(level) if level == state.highest_level => {
                // state.flush_cur_node()?;
                state.in_highest_level = true;
            }
            IntermediateToken::Value(value) => {
                if state.in_highest_level {
                    state.push_cur_node(AstNode::Value(value));
                }
                else {
                    state.push_token(IntermediateToken::AstNode(AstNode::Value(value)))
                }
            },
            IntermediateToken::AstNode(ast_node) if state.in_highest_level => state.push_cur_node(ast_node),
            IntermediateToken::RParen(level) if level == state.highest_level => {
                state.flush_cur_node()?;
                state.in_highest_level = false;
            },
            IntermediateToken::EOF => (),
            a => state.push_token(a)
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct FinalParser {}

impl FinalParser {
    fn parsing_is_done(tokens: &Vec<IntermediateToken>) -> bool {
        tokens.iter()
            .all(|x| matches!(x, IntermediateToken::AstNode(_)))
    }

    fn complete_parsing(tokens: Vec<IntermediateToken>) -> Result<Vec<AstNode>, CIParserError> {
        tokens
            .into_iter()
            .map(|x| match x {
                IntermediateToken::AstNode(a) => Ok(a),
                _ => Err(CIParserError::ParsingUnfinished)
            })
            .collect()
    }
}

impl Parser for FinalParser {
    type InputNode = IntermediateToken;
    type OutputNode = AstNode;

    fn parse(&self, tokens: Vec<Self::InputNode>) -> Result<Vec<AstNode>, CIParserError> {
        let mut tokens = tokens;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000;
        
        while !Self::parsing_is_done(&tokens) {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                return Err(CIParserError::ParsingUnfinished);
            }
            
            let parser_step = ParserStep::default();
            tokens = parser_step.parse(tokens)?
        }
        Ok(Self::complete_parsing(tokens)?)
    }
}

pub type CICoreParser = SeqParsers<IntermediateTokenizer, FinalParser>;
