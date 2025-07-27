use crate::{ast::{AstNode, IntermediateToken}, parser_types::{CIParserError, Parser}, parsers::ParserStep};

#[derive(Default)]
pub struct CIFinalParserRepl {}

impl CIFinalParserRepl {
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
                // .nth(0).unwrap()
    }
}

impl Parser for CIFinalParserRepl {
    type Input = Vec<IntermediateToken>;
    type Output = Vec<AstNode>;

    fn parse(&self, tokens: Self::Input) -> Result<Vec<AstNode>, CIParserError> {
        let mut tokens = tokens;
        
        while !Self::parsing_is_done(&tokens) {
            let parser_step = ParserStep::default();
            tokens = parser_step.parse(tokens)?
        }

        Ok(Self::complete_parsing(tokens)?)
    }
}
