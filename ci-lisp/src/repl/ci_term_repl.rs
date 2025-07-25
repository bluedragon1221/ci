use std::cell::RefCell;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

use crate::{
    parser_types::Parser, repl::{CIReplError, ReadSignal, Repl}
};

pub struct CITermRepl<P> {
    line_editor: RefCell<Reedline>,
    prompt: DefaultPrompt,

    parser: P
}

impl<P: Default> Default for CITermRepl<P> {
    fn default() -> Self {
        Self {
            line_editor: RefCell::new(Reedline::create()),
            prompt: DefaultPrompt::new(
                DefaultPromptSegment::Empty,
                DefaultPromptSegment::Empty
            ),

            parser: P::default()
        }
    }
}

impl<P> Repl for CITermRepl<P>
where
    P: Parser<InputNode = char, OutputNode: std::fmt::Debug>
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
        Ok(self.parser.parse(input.chars().collect())?)
    }

    fn print(&self, output: Self::Output) -> Result<(), CIReplError> {
        Ok(println!("{:#?}", output))
    }
}
