use crate::{ast::AstNode, env::Environment, parser_types::{CIParserError, Parser}, parsers::CIFileEvaluator};

pub struct CIReplEvaluator {
    preload: Vec<String>,
    file_evaluator: CIFileEvaluator
}

impl CIReplEvaluator {
    pub fn new(preload: Vec<String>, initial_env: Environment) -> Self {
        Self {
            preload,
            file_evaluator: CIFileEvaluator::new(initial_env)
        }
    }
}

impl Parser for CIReplEvaluator {
    type Input = AstNode;
    type Output = AstNode;

    fn parse(&self, ast: AstNode) -> Result<AstNode, CIParserError> {
        for i in self.preload.iter() {
            self.file_evaluator.load_file(i.to_string())?;
        }

        Ok(self.file_evaluator.parse(vec![ast])?.pop().ok_or(CIParserError::ParsingUnfinished)?)
    }
}
