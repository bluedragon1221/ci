mod ci_lexer;
pub use ci_lexer::{CILexerError, CILexer};

mod ci_intermediate_tokenizer;
pub use ci_intermediate_tokenizer::CIIntermediateTokenizer;

// mod ci_parser_step;
// pub use ci_parser_step::ParserStep;

// mod ci_final_parser;
// pub use ci_final_parser::CIFinalParser;

mod ci_evaluator;
pub use ci_evaluator::{CIEvalError, CIFileEvaluator};

// mod ci_final_parser_repl;
// pub use ci_final_parser_repl::CIFinalParserRepl;

mod ci_repl_evaluator;
pub use ci_repl_evaluator::CIReplEvaluator;

mod ci_new_parser;
pub use ci_new_parser::{CINewReplParser, CINewFileParser};

// ---

use crate::{ast::AstNode, parser_types::SeqParsers};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type CIFullFileParser = SeqParsers<SeqParsers<CILexer, CIIntermediateTokenizer>, CINewFileParser>;
pub type CIFullFileEvaluator = SeqParsers<CIFullFileParser, CIFileEvaluator>;

pub fn ci_full_file_evaluator_with_env(custom_env: Rc<RefCell<HashMap<String, AstNode>>>) -> CIFullFileEvaluator {
    SeqParsers::new(
        CIFullFileParser::default(),
        CIFileEvaluator::new(custom_env)
    )
}

