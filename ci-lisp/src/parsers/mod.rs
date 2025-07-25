mod ci_lexer;
pub use ci_lexer::{CILexerError, CILexer};

mod ci_intermediate_tokenizer;
pub use ci_intermediate_tokenizer::CIIntermediateTokenizer;

mod ci_parser_step;
pub use ci_parser_step::ParserStep;

mod ci_final_parser;
pub use ci_final_parser::CIFinalParser;

mod ci_evaluator;
pub use ci_evaluator::{CIEvalError, CIEvaluator};

// ---

use crate::{ast::AstNode, parser_types::SeqParsers};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type CIOnlyParsing = SeqParsers<SeqParsers<CILexer, CIIntermediateTokenizer>, CIFinalParser>;
pub type CIParser = SeqParsers<CIOnlyParsing, CIEvaluator>;

pub fn ci_parser_with_env(custom_env: Rc<RefCell<HashMap<String, AstNode>>>) -> CIParser {
    SeqParsers::new(
        SeqParsers::<SeqParsers<CILexer, CIIntermediateTokenizer>, CIFinalParser>::default(),
        CIEvaluator::new(custom_env)
    )
}

