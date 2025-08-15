use crate::{ast::{AstNode, Value}, env::Environment, parser_types::{CIParserError, Parser}, parsers::CIFileEvaluator};

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
            self.file_evaluator.parse(vec![AstNode::Par {
                car: Box::new(AstNode::Value(Value::Symbol("include".to_string()))),
                cdr: Box::new(AstNode::Value(Value::String(i.to_string())))
            }])?;
        }

        Ok(self.file_evaluator.parse(vec![ast])?[0].clone())
    }
}
