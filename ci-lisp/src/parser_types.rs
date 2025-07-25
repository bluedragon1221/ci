use crate::{ast::{AstNode, IntermediateToken, Token}, parsers::{CIEvalError, CILexerError}};

#[derive(Debug, thiserror::Error)]
pub enum CIParserError {
    #[error("Missing Opening Parenthesis (level: {0})")]
    MissingOpenParen(i32),

    #[error("Missing Closing Parenthesis (level: {0})")]
    MissingCloseParen(i32),
    
    #[error("UnknownToken: {0:?}")]
    UnknownToken(Box<Token>),

    #[error("UnexpectedToken: {0:?}")]
    UnexpectedToken(Box<IntermediateToken>),

    #[error("Too many parameters in Node: {0:?}")]
    NodeFull(Vec<AstNode>),

    #[error("[Internal] parsing not done")]
    ParsingUnfinished,

    #[error("LexerError: {0}")]
    LexerError(#[from] CILexerError),

    #[error("EvalError: {0}")]
    EvalError(#[from] CIEvalError)
}

pub trait Parser {
    type InputNode;
    type OutputNode;

    fn parse(&self, tokens: Vec<Self::InputNode>) -> Result<Vec<Self::OutputNode>, CIParserError>;
}

pub trait SingleParser {
    type InputNode;
    type OutputNode;
    type State: ParserState<OutputNode = Self::OutputNode>;

    fn handle_token(token: Self::InputNode, state: &mut Self::State) -> Result<(), CIParserError>;

    fn init_state(tokens: &Vec<Self::InputNode>) -> Self::State;

    fn single_parse(&self, tokens: Vec<Self::InputNode>) -> Result<Vec<Self::OutputNode>, CIParserError> {
        let mut state = Self::init_state(&tokens);

        for token in tokens.into_iter() {
            Self::handle_token(token, &mut state)?
        }

        Ok(state.take_tokens())
    }
}
impl<T: SingleParser> Parser for T {
    type InputNode = T::InputNode;
    type OutputNode = T::OutputNode;

    fn parse(&self, tokens: Vec<Self::InputNode>) -> Result<Vec<Self::OutputNode>, CIParserError> {
        T::single_parse(&self, tokens)
    }
}

pub trait SingleParserDefault {
    type InputNode;
    type OutputNode;
    type State: ParserState<OutputNode = Self::OutputNode> + Default;
    
    fn handle_token(token: Self::InputNode, state: &mut Self::State) -> Result<(), CIParserError>;
}
impl<T> SingleParser for T 
where 
    T: SingleParserDefault,
{
    type InputNode = T::InputNode;
    type OutputNode = T::OutputNode;
    type State = T::State;
    
    fn handle_token(token: Self::InputNode, state: &mut Self::State) -> Result<(), CIParserError> {
        T::handle_token(token, state)
    }
    
    fn init_state(_tokens: &Vec<Self::InputNode>) -> Self::State {
        Self::State::default()
    }
}


pub trait ParserState {
    type OutputNode;

    fn take_tokens(self) -> Vec<Self::OutputNode>;
}

#[derive(Default)]
pub struct SeqParsers<A, B> {
    a: A,
    b: B
}
impl<A, B> Parser for SeqParsers<A, B>
where
    A: Parser,
    B: Parser<InputNode = A::OutputNode>
{
    type InputNode = A::InputNode;
    type OutputNode = B::OutputNode;

    fn parse(&self, tokens: Vec<Self::InputNode>) -> Result<Vec<Self::OutputNode>, CIParserError> {
        let tokens = self.a.parse(tokens)?;
        self.b.parse(tokens)
    }
}

impl<A, B> SeqParsers<A, B> {
    pub fn new(a: A, b: B) -> SeqParsers<A, B> {
        SeqParsers { a, b }
    }
}
