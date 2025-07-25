use crate::{ast::{AstNode, IntermediateToken}, parser_types::{CIParserError, Parser}, parsers::ParserStep};

#[derive(Default)]
pub struct CIFinalParser {}

impl CIFinalParser {
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

impl Parser for CIFinalParser {
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
