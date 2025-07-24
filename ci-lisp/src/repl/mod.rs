#[derive(Debug, thiserror::Error)]
pub enum CIReplError {
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    #[error("LexerError: {0}")]
    LexerError(#[from] crate::lexer::CILexerError),

    #[error("ParserError: {0}")]
    ParserError(#[from] crate::parser::CIParserError)
}

pub enum ReadSignal<InputType> {
    Input(InputType),
    Nothing,
    Quit
}

pub trait Repl {
    type Input;
    type Output;

    fn read(&self) -> Result<ReadSignal<Self::Input>, CIReplError>;
    fn evaluate(&self, input: Self::Input) -> Result<Self::Output, CIReplError>;
    fn print(&self, output: Self::Output) -> Result<(), CIReplError>;
    fn r#loop(&self) {
        loop {
            match self.read() {
                Ok(ReadSignal::Input(input)) => {
                    match self.evaluate(input) {
                        Ok(a) => self.print(a).unwrap(),
                        Err(e) => eprintln!("{}", e)
                    }
                },
                Ok(ReadSignal::Quit) => break,
                Ok(ReadSignal::Nothing) => (),
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}

mod ci_term_repl;
pub use ci_term_repl::{CITermRepl};
