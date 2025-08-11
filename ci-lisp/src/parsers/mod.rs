mod ci_lexer;
pub use ci_lexer::{CILexerError, CILexer};

mod ci_streaming_lexer;
pub use ci_streaming_lexer::CIStreamingLexer;

mod ci_intermediate_tokenizer;
pub use ci_intermediate_tokenizer::CIIntermediateTokenizer;

mod ci_evaluator;
pub use ci_evaluator::{CIEvalError, CIFileEvaluator};

mod ci_repl_evaluator;
pub use ci_repl_evaluator::CIReplEvaluator;

mod ci_new_parser;
pub use ci_new_parser::{CINewReplParser, CINewFileParser};

// ---

use crate::parser_types::SeqParsers;
pub type CIFullFileParser = SeqParsers<SeqParsers<CILexer, CIIntermediateTokenizer>, CINewFileParser>;
pub type CIFullFileEvaluator = SeqParsers<CIFullFileParser, CIFileEvaluator>;
