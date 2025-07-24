pub mod repl;
pub mod lexer;
pub mod parser;
pub mod ast;

use repl::CIReplError;

#[derive(Debug, thiserror::Error)]
pub enum CIError {
    #[error("ReplError: {0}")]
    ReplError(#[from] CIReplError),
}

