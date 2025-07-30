use std::cell::RefCell;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

use crate::{parser_types::Parser, repl::{CIReplError, ReadSignal, Repl}};

pub struct CITermRepl<P> {
    line_editor: RefCell<Reedline>,
    prompt: DefaultPrompt,

    parser: P,
}

impl<P> CITermRepl<P> {
    pub fn new(parser: P) -> Self {
        Self {
            line_editor: RefCell::new(Reedline::create()),
            prompt: DefaultPrompt::new(
                DefaultPromptSegment::Empty,
                DefaultPromptSegment::Empty
            ),
            parser,
        }
    }
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

impl<P, O> Repl for CITermRepl<P>
where
    O: IntoIterator<Item: std::fmt::Display>,
    P: Parser<Input = String, Output = O>
{
    type Input = String;
    type Output = O;

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
