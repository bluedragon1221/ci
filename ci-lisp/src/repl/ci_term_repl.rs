use std::cell::RefCell;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

use crate::{
    parser_types::Parser, repl::{CIReplError, ReadSignal, Repl}
};

pub struct CITermRepl<P>
where
    P: Parser<Input = String, Output: IntoIterator<Item: std::fmt::Display>>
{
    line_editor: RefCell<Reedline>,
    prompt: DefaultPrompt,

    parser: P
}

impl<P> CITermRepl<P>
where
    P: Parser<Input = String, Output: IntoIterator<Item: std::fmt::Display>>
{
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            line_editor: RefCell::new(Reedline::create()),
            prompt: DefaultPrompt::new(
                DefaultPromptSegment::Empty,
                DefaultPromptSegment::Empty
            ),
        }
    }
}

impl<P> Default for CITermRepl<P>
where
    P: Parser<Input = String, Output: IntoIterator<Item: std::fmt::Display>> + Default
{
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
    P: Parser<Input = String, Output: IntoIterator<Item: std::fmt::Display>>
{
    type Input = String;
    type Output = P::Output;

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
        Ok(self.parser.parse(input)?)
    }

    fn print(&self, output: Self::Output) -> Result<(), CIReplError> {
        for i in output.into_iter() {
            println!("{}", i);
        }

        Ok(())
    }
}
