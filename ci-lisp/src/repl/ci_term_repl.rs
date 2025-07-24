use std::cell::RefCell;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

use crate::{
    lexer::{Lexer, Token},
    parser::Parser,
    repl::{CIReplError, ReadSignal, Repl}
};

pub struct CITermRepl<L, P> {
    line_editor: RefCell<Reedline>,
    prompt: DefaultPrompt,

    lexer: L,
    parser: P
}

impl<L: Default, P: Default> Default for CITermRepl<L, P> {
    fn default() -> Self {
        Self {
            line_editor: RefCell::new(Reedline::create()),
            prompt: DefaultPrompt::new(
                DefaultPromptSegment::Empty,
                DefaultPromptSegment::Empty
            ),

            lexer: L::default(),
            parser: P::default()
        }
    }
}

impl<L, P> Repl for CITermRepl<L, P>
where
    L: Lexer<Input = String>,
    P: Parser<InputNode = Token, OutputNode: std::fmt::Debug>
{
    type Input = String;
    type Output = Vec<P::OutputNode>;

    fn read(&self) -> Result<ReadSignal<Self::Input>, CIReplError> {
        let mut line_editor = self.line_editor.borrow_mut();

        let sig = line_editor.read_line(&self.prompt)?;
        match sig {
            Signal::Success(a) if a == String::from("") => Ok(ReadSignal::Nothing),
            Signal::Success(buffer) => Ok(ReadSignal::Input(buffer)),
            Signal::CtrlD | Signal::CtrlC => Ok(ReadSignal::Quit),
        }
    }

    fn evaluate(&self, input: String) -> Result<Self::Output, CIReplError> {
        let tokens = self.lexer.tokenize(input)?;

        Ok(self.parser.parse(tokens)?)
    }

    fn print(&self, output: Self::Output) -> Result<(), CIReplError> {
        Ok(println!("{:#?}", output))
    }
}
