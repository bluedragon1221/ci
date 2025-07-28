use crate::{ast::{AstNode, Value}, parser_types::{CIParserError, Parser}, parsers::CIFileEvaluator};

#[derive(Default)]
pub struct CIReplEvaluator {
    file_evaluator: CIFileEvaluator,

    preload: Vec<String>
}

impl CIReplEvaluator {
    pub fn new(preload: Vec<String>) -> Self {
        Self {
            preload,
            ..Default::default()
        }
    }
}

impl Parser for CIReplEvaluator {
    type Input = AstNode;
    type Output = Vec<AstNode>;

    fn parse(&self, ast: AstNode) -> Result<Vec<AstNode>, CIParserError> {
        for i in self.preload.iter() {
            self.file_evaluator.parse(vec![AstNode::Par {
                car: Box::new(AstNode::Value(Value::Symbol("include".to_string()))),
                cdr: Box::new(AstNode::Value(Value::String(i.to_string())))
            }])?;
        }

        Ok(self.file_evaluator.parse(vec![ast])?)
    }
}
