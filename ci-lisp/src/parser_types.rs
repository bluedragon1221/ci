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
    EvalError(#[from] CIEvalError),
}

pub trait Parser {
    type Input;
    type Output;

    fn parse(&self, tokens: Self::Input) -> Result<Self::Output, CIParserError>;
}

pub trait ParserState {
    type Output;

    fn take_tokens(self) -> Self::Output;
}

pub trait SingleParser {
    type Input: IntoIterator;
    type Output;
    type State: ParserState<Output = Self::Output>;

    fn handle_token(token: <Self::Input as IntoIterator>::Item, state: &mut Self::State) -> Result<(), CIParserError>;

    fn init_state(tokens: &Self::Input) -> Self::State;

    fn single_parse(&self, tokens: Self::Input) -> Result<Self::Output, CIParserError> {
        let mut state = Self::init_state(&tokens);

        for token in tokens.into_iter() {
            Self::handle_token(token, &mut state)?
        }

        Ok(state.take_tokens())
    }
}
impl<T: SingleParser> Parser for T {
    type Input = T::Input;
    type Output = T::Output;

    fn parse(&self, tokens: Self::Input) -> Result<Self::Output, CIParserError> {
        T::single_parse(&self, tokens)
    }
}

pub trait SingleParserDefault {
    type Input: IntoIterator;
    type Output;
    type State: ParserState<Output = Self::Output> + Default;
    
    fn handle_token(token: <Self::Input as IntoIterator>::Item, state: &mut Self::State) -> Result<(), CIParserError>;
}
impl<T> SingleParser for T 
where 
    T: SingleParserDefault,
{
    type Input = T::Input;
    type Output = T::Output;
    type State = T::State;
    
    fn handle_token(token: <T::Input as IntoIterator>::Item, state: &mut Self::State) -> Result<(), CIParserError> {
        T::handle_token(token, state)
    }
    
    fn init_state(_tokens: &Self::Input) -> Self::State {
        Self::State::default()
    }
}


#[derive(Default)]
pub struct SeqParsers<A, B>
where
    A: Parser,
    B: Parser<Input = A::Output>
{
    a: A,
    b: B
}

impl<A, B> Parser for SeqParsers<A, B>
where
    A: Parser,
    B: Parser<Input = A::Output>
{
    type Input = A::Input;
    type Output = B::Output;

    fn parse(&self, tokens: Self::Input) -> Result<Self::Output, CIParserError> {
        let tokens = self.a.parse(tokens)?;
        self.b.parse(tokens)
    }
}

impl<A, B> SeqParsers<A, B>
where
    A: Parser,
    B: Parser<Input = A::Output>
{
    pub fn new(a: A, b: B) -> SeqParsers<A, B> {
        SeqParsers { a, b }
    }
}
